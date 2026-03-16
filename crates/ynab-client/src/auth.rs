use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use base64::Engine;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use rand::Rng;
use sha2::{Digest, Sha256};

use crate::error::YnabError;

const KEYRING_SERVICE: &str = "ynab-cli";
const KEYRING_USER_ACCESS: &str = "access-token";
const KEYRING_USER_REFRESH: &str = "refresh-token";
const KEYRING_USER_EXPIRES: &str = "token-expires-at";
const KEYRING_USER_TYPE: &str = "token-type";

// OAuth2 client credentials (embedded for public CLI, standard pattern)
pub const OAUTH_CLIENT_ID: &str = "fAQ47EqjUsClke4mdbjuf7P9inaxOk0eYJQCl86na2Y";
pub const OAUTH_CLIENT_SECRET: &str = "rtfse_Eii_OT7mjcFlB2kUW62ts7y9UO3Acdce0gnEE";
pub const OAUTH_REDIRECT_URI: &str = "http://localhost:14512/callback";
pub const OAUTH_AUTHORIZE_URL: &str = "https://app.ynab.com/oauth/authorize";
pub const OAUTH_TOKEN_URL: &str = "https://app.ynab.com/oauth/token";

/// Token refresh buffer — refresh 5 minutes before actual expiry.
const REFRESH_BUFFER_SECS: u64 = 300;

// --- PKCE helpers ---

/// Generate a random PKCE code verifier (43-128 chars, base64url).
pub fn generate_code_verifier() -> String {
    let mut rng = rand::rng();
    let bytes: Vec<u8> = (0..64).map(|_| rng.random::<u8>()).collect();
    URL_SAFE_NO_PAD.encode(&bytes)
}

/// Compute the S256 code challenge from a code verifier.
pub fn compute_code_challenge(verifier: &str) -> String {
    let digest = Sha256::digest(verifier.as_bytes());
    URL_SAFE_NO_PAD.encode(digest)
}

/// Generate a random state parameter for CSRF protection.
pub fn generate_state() -> String {
    let mut rng = rand::rng();
    let bytes: Vec<u8> = (0..32).map(|_| rng.random::<u8>()).collect();
    URL_SAFE_NO_PAD.encode(&bytes)
}

// --- Token resolution ---

/// Resolves the YNAB access token from available sources.
///
/// Resolution order:
/// 1. `YNAB_ACCESS_TOKEN` environment variable
/// 2. Explicit token passed via `--token` flag
/// 3. OS keychain (macOS Keychain, Linux Secret Service, Windows Credential Manager)
/// 4. File-based fallback (~/.config/ynab/credentials.json)
///
/// For OAuth tokens, auto-refreshes if expired.
pub fn resolve_token(explicit_token: Option<&str>) -> Result<String, YnabError> {
    // 1. Environment variable
    if let Ok(token) = std::env::var("YNAB_ACCESS_TOKEN")
        && !token.is_empty()
    {
        return Ok(token);
    }

    // 2. Explicit token
    if let Some(token) = explicit_token {
        return Ok(token.to_string());
    }

    // 3. OS keychain
    if let Some(token) = load_keychain_token() {
        if is_oauth_keychain() && is_token_expired_keychain() {
            return refresh_token_from_keychain();
        }
        return Ok(token);
    }

    // 4. File-based fallback
    if let Some(creds) = load_file_credentials()? {
        if creds.token_type == "oauth"
            && let Some(expires_at) = creds.expires_at
            && is_expired(expires_at)
            && let Some(refresh) = &creds.refresh_token
        {
            return refresh_and_store(refresh);
        }
        return Ok(creds.access_token);
    }

    Err(YnabError::NotAuthenticated)
}

/// Resolve token asynchronously (needed for token refresh which makes HTTP calls).
/// This is the preferred method for commands that already have an async context.
pub async fn resolve_token_async(explicit_token: Option<&str>) -> Result<String, YnabError> {
    // 1. Environment variable
    if let Ok(token) = std::env::var("YNAB_ACCESS_TOKEN")
        && !token.is_empty()
    {
        return Ok(token);
    }

    // 2. Explicit token
    if let Some(token) = explicit_token {
        return Ok(token.to_string());
    }

    // 3. OS keychain
    if let Some(token) = load_keychain_token() {
        if is_oauth_keychain() && is_token_expired_keychain() {
            return refresh_token_from_keychain_async().await;
        }
        return Ok(token);
    }

    // 4. File-based fallback
    if let Some(creds) = load_file_credentials()? {
        if creds.token_type == "oauth"
            && let Some(expires_at) = creds.expires_at
            && is_expired(expires_at)
            && let Some(refresh) = &creds.refresh_token
        {
            return refresh_and_store_async(refresh).await;
        }
        return Ok(creds.access_token);
    }

    Err(YnabError::NotAuthenticated)
}

// --- Token storage ---

/// Store a personal access token. Prefers OS keychain, falls back to file.
pub fn store_token(token: &str) -> Result<(), YnabError> {
    if store_keychain_pat(token) {
        return Ok(());
    }

    eprintln!("Warning: OS keychain unavailable, storing token in config file");
    store_file_credentials(&StoredCredentials {
        token_type: "pat".into(),
        access_token: token.into(),
        refresh_token: None,
        expires_at: None,
    })
}

/// Store OAuth tokens. Prefers OS keychain, falls back to file.
pub fn store_oauth_tokens(
    access_token: &str,
    refresh_token: &str,
    expires_in: u64,
) -> Result<(), YnabError> {
    let expires_at = now_secs() + expires_in;

    if store_keychain_oauth(access_token, refresh_token, expires_at) {
        return Ok(());
    }

    eprintln!("Warning: OS keychain unavailable, storing tokens in config file");
    store_file_credentials(&StoredCredentials {
        token_type: "oauth".into(),
        access_token: access_token.into(),
        refresh_token: Some(refresh_token.into()),
        expires_at: Some(expires_at),
    })
}

/// Clear stored credentials from both keychain and file.
pub fn clear_token() -> Result<(), YnabError> {
    // Clear all keychain entries
    for user in [
        KEYRING_USER_ACCESS,
        KEYRING_USER_REFRESH,
        KEYRING_USER_EXPIRES,
        KEYRING_USER_TYPE,
    ] {
        if let Ok(entry) = keyring::Entry::new(KEYRING_SERVICE, user) {
            let _ = entry.delete_credential();
        }
    }

    let path = credentials_path()?;
    if path.exists() {
        std::fs::remove_file(&path)
            .map_err(|e| YnabError::Config(format!("Failed to remove credentials file: {e}")))?;
    }

    Ok(())
}

/// Check if a token is available without returning it.
pub fn has_token() -> bool {
    resolve_token(None).is_ok()
}

/// Returns where the token is currently stored.
pub fn token_storage_type() -> &'static str {
    if load_keychain_token().is_some() {
        return "keychain";
    }
    if load_file_credentials().ok().flatten().is_some() {
        return "file";
    }
    "none"
}

/// Returns the credential type (pat, oauth, or none).
pub fn credential_type() -> &'static str {
    if is_oauth_keychain() {
        return "oauth";
    }
    if load_keychain_token().is_some() {
        return "pat";
    }
    if let Ok(Some(creds)) = load_file_credentials() {
        if creds.token_type == "oauth" {
            return "oauth";
        }
        return "pat";
    }
    "none"
}

// --- OAuth token exchange ---

/// Exchange an authorization code for tokens.
pub async fn exchange_code(
    code: &str,
    code_verifier: &str,
) -> Result<OAuthTokenResponse, YnabError> {
    let client = reqwest::Client::new();
    let resp = client
        .post(OAUTH_TOKEN_URL)
        .form(&[
            ("client_id", OAUTH_CLIENT_ID),
            ("client_secret", OAUTH_CLIENT_SECRET),
            ("grant_type", "authorization_code"),
            ("code", code),
            ("redirect_uri", OAUTH_REDIRECT_URI),
            ("code_verifier", code_verifier),
        ])
        .send()
        .await
        .map_err(|e| YnabError::Other(format!("Token exchange failed: {e}")))?;

    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp.text().await.unwrap_or_else(|_| "unknown".to_string());
        return Err(YnabError::Other(format!(
            "Token exchange failed ({status}): {body}"
        )));
    }

    resp.json::<OAuthTokenResponse>()
        .await
        .map_err(|e| YnabError::Other(format!("Failed to parse token response: {e}")))
}

#[derive(Debug, serde::Deserialize)]
pub struct OAuthTokenResponse {
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub expires_in: Option<u64>,
    pub token_type: Option<String>,
}

// --- Internal helpers ---

#[derive(Debug)]
struct StoredCredentials {
    token_type: String,
    access_token: String,
    refresh_token: Option<String>,
    expires_at: Option<u64>,
}

fn now_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

fn is_expired(expires_at: u64) -> bool {
    now_secs() + REFRESH_BUFFER_SECS >= expires_at
}

// --- Keychain operations ---

fn load_keychain_token() -> Option<String> {
    let entry = keyring::Entry::new(KEYRING_SERVICE, KEYRING_USER_ACCESS).ok()?;
    entry.get_password().ok()
}

fn is_oauth_keychain() -> bool {
    keyring::Entry::new(KEYRING_SERVICE, KEYRING_USER_TYPE)
        .ok()
        .and_then(|e| e.get_password().ok())
        .is_some_and(|t| t == "oauth")
}

fn is_token_expired_keychain() -> bool {
    let Some(expires_str) = keyring::Entry::new(KEYRING_SERVICE, KEYRING_USER_EXPIRES)
        .ok()
        .and_then(|e| e.get_password().ok())
    else {
        return false;
    };
    let Ok(expires_at) = expires_str.parse::<u64>() else {
        return false;
    };
    is_expired(expires_at)
}

fn load_keychain_refresh_token() -> Option<String> {
    keyring::Entry::new(KEYRING_SERVICE, KEYRING_USER_REFRESH)
        .ok()
        .and_then(|e| e.get_password().ok())
}

fn store_keychain_pat(token: &str) -> bool {
    let Ok(entry) = keyring::Entry::new(KEYRING_SERVICE, KEYRING_USER_ACCESS) else {
        return false;
    };
    if entry.set_password(token).is_err() {
        return false;
    }
    // Set type to pat
    if let Ok(type_entry) = keyring::Entry::new(KEYRING_SERVICE, KEYRING_USER_TYPE) {
        let _ = type_entry.set_password("pat");
    }
    // Clean up any OAuth-specific entries
    for user in [KEYRING_USER_REFRESH, KEYRING_USER_EXPIRES] {
        if let Ok(e) = keyring::Entry::new(KEYRING_SERVICE, user) {
            let _ = e.delete_credential();
        }
    }
    true
}

fn store_keychain_oauth(access_token: &str, refresh_token: &str, expires_at: u64) -> bool {
    let entries = [
        (KEYRING_USER_ACCESS, access_token.to_string()),
        (KEYRING_USER_REFRESH, refresh_token.to_string()),
        (KEYRING_USER_EXPIRES, expires_at.to_string()),
        (KEYRING_USER_TYPE, "oauth".to_string()),
    ];

    for (user, value) in &entries {
        let Ok(entry) = keyring::Entry::new(KEYRING_SERVICE, user) else {
            return false;
        };
        if entry.set_password(value).is_err() {
            return false;
        }
    }
    true
}

fn refresh_token_from_keychain() -> Result<String, YnabError> {
    let refresh = load_keychain_refresh_token()
        .ok_or_else(|| YnabError::Other("No refresh token in keychain".into()))?;
    refresh_and_store(&refresh)
}

async fn refresh_token_from_keychain_async() -> Result<String, YnabError> {
    let refresh = load_keychain_refresh_token()
        .ok_or_else(|| YnabError::Other("No refresh token in keychain".into()))?;
    refresh_and_store_async(&refresh).await
}

/// Synchronous token refresh (blocking).
fn refresh_and_store(refresh_token: &str) -> Result<String, YnabError> {
    let rt = tokio::runtime::Handle::try_current();
    match rt {
        Ok(_handle) => {
            // We're inside a tokio runtime — spawn a blocking task
            std::thread::scope(|s| {
                s.spawn(|| {
                    let new_rt = tokio::runtime::Runtime::new()
                        .map_err(|e| YnabError::Other(format!("Failed to create runtime: {e}")))?;
                    new_rt.block_on(refresh_and_store_async(refresh_token))
                })
                .join()
                .unwrap()
            })
        }
        Err(_) => {
            // No runtime — create one
            let rt = tokio::runtime::Runtime::new()
                .map_err(|e| YnabError::Other(format!("Failed to create runtime: {e}")))?;
            rt.block_on(refresh_and_store_async(refresh_token))
        }
    }
}

/// Async token refresh.
async fn refresh_and_store_async(refresh_token: &str) -> Result<String, YnabError> {
    let client = reqwest::Client::new();
    let resp = client
        .post(OAUTH_TOKEN_URL)
        .form(&[
            ("client_id", OAUTH_CLIENT_ID),
            ("client_secret", OAUTH_CLIENT_SECRET),
            ("grant_type", "refresh_token"),
            ("refresh_token", refresh_token),
        ])
        .send()
        .await
        .map_err(|e| YnabError::Other(format!("Token refresh failed: {e}")))?;

    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp.text().await.unwrap_or_else(|_| "unknown".to_string());
        return Err(YnabError::Other(format!(
            "Token refresh failed ({status}): {body}. Run `ynab auth login` to re-authenticate."
        )));
    }

    let token_resp: OAuthTokenResponse = resp
        .json()
        .await
        .map_err(|e| YnabError::Other(format!("Failed to parse refresh response: {e}")))?;

    let new_refresh = token_resp.refresh_token.as_deref().unwrap_or(refresh_token);
    let expires_in = token_resp.expires_in.unwrap_or(7200);

    store_oauth_tokens(&token_resp.access_token, new_refresh, expires_in)?;

    Ok(token_resp.access_token)
}

// --- File-based fallback ---

fn load_file_credentials() -> Result<Option<StoredCredentials>, YnabError> {
    let path = credentials_path()?;
    if !path.exists() {
        return Ok(None);
    }

    let content = std::fs::read_to_string(&path)
        .map_err(|e| YnabError::Config(format!("Failed to read credentials: {e}")))?;

    let creds: serde_json::Value = serde_json::from_str(&content)?;

    let access_token = creds
        .get("access_token")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());

    let Some(access_token) = access_token else {
        return Ok(None);
    };

    Ok(Some(StoredCredentials {
        token_type: creds
            .get("type")
            .and_then(|v| v.as_str())
            .unwrap_or("pat")
            .to_string(),
        access_token,
        refresh_token: creds
            .get("refresh_token")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string()),
        expires_at: creds.get("expires_at").and_then(|v| v.as_u64()),
    }))
}

fn store_file_credentials(creds: &StoredCredentials) -> Result<(), YnabError> {
    let path = credentials_path()?;
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| YnabError::Config(format!("Failed to create config dir: {e}")))?;
    }

    let mut json = serde_json::json!({
        "type": creds.token_type,
        "access_token": creds.access_token,
    });

    if let Some(ref refresh) = creds.refresh_token {
        json["refresh_token"] = serde_json::json!(refresh);
    }
    if let Some(expires_at) = creds.expires_at {
        json["expires_at"] = serde_json::json!(expires_at);
    }

    std::fs::write(&path, serde_json::to_string_pretty(&json)?)
        .map_err(|e| YnabError::Config(format!("Failed to write credentials: {e}")))?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o600))
            .map_err(|e| YnabError::Config(format!("Failed to set permissions: {e}")))?;
    }

    Ok(())
}

fn credentials_path() -> Result<PathBuf, YnabError> {
    let config_dir = dirs::config_dir()
        .ok_or_else(|| YnabError::Config("Could not determine config directory".into()))?;
    Ok(config_dir.join("ynab").join("credentials.json"))
}
