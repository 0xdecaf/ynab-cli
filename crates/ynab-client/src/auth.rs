use std::path::PathBuf;

use crate::error::YnabError;

const KEYRING_SERVICE: &str = "ynab-cli";
const KEYRING_USER: &str = "access-token";

/// Resolves the YNAB access token from available sources.
///
/// Resolution order:
/// 1. `YNAB_ACCESS_TOKEN` environment variable
/// 2. Explicit token passed via `--token` flag
/// 3. OS keychain (macOS Keychain, Linux Secret Service, Windows Credential Manager)
/// 4. File-based fallback (~/.config/ynab/credentials.json)
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
        return Ok(token);
    }

    // 4. File-based fallback
    if let Some(token) = load_file_token()? {
        return Ok(token);
    }

    Err(YnabError::NotAuthenticated)
}

/// Store a personal access token. Prefers OS keychain, falls back to file.
pub fn store_token(token: &str) -> Result<(), YnabError> {
    if store_keychain_token(token) {
        return Ok(());
    }

    eprintln!("Warning: OS keychain unavailable, storing token in config file");
    store_file_token(token)
}

/// Clear stored credentials from both keychain and file.
pub fn clear_token() -> Result<(), YnabError> {
    if let Ok(entry) = keyring::Entry::new(KEYRING_SERVICE, KEYRING_USER) {
        let _ = entry.delete_credential();
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
    if load_file_token().ok().flatten().is_some() {
        return "file";
    }
    "none"
}

// --- Keychain operations ---

fn load_keychain_token() -> Option<String> {
    let entry = keyring::Entry::new(KEYRING_SERVICE, KEYRING_USER).ok()?;
    entry.get_password().ok()
}

fn store_keychain_token(token: &str) -> bool {
    let Ok(entry) = keyring::Entry::new(KEYRING_SERVICE, KEYRING_USER) else {
        return false;
    };
    entry.set_password(token).is_ok()
}

// --- File-based fallback ---

fn load_file_token() -> Result<Option<String>, YnabError> {
    let path = credentials_path()?;
    if !path.exists() {
        return Ok(None);
    }

    let content = std::fs::read_to_string(&path)
        .map_err(|e| YnabError::Config(format!("Failed to read credentials: {e}")))?;

    let creds: serde_json::Value = serde_json::from_str(&content)?;
    Ok(creds
        .get("access_token")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string()))
}

fn store_file_token(token: &str) -> Result<(), YnabError> {
    let path = credentials_path()?;
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| YnabError::Config(format!("Failed to create config dir: {e}")))?;
    }

    let creds = serde_json::json!({
        "type": "pat",
        "access_token": token,
    });

    std::fs::write(&path, serde_json::to_string_pretty(&creds)?)
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
