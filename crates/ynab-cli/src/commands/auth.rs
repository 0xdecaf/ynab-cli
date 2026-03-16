use std::io::{BufRead, BufReader, Write};
use std::net::TcpListener;

use anyhow::Result;
use url::Url;
use ynab_client::auth;

use crate::cli::AuthCommand;

pub async fn run(command: &AuthCommand) -> Result<()> {
    match command {
        AuthCommand::Login { pat, token } => {
            if *pat {
                login_pat(token.as_deref())?;
            } else {
                login_oauth().await?;
            }
            Ok(())
        }

        AuthCommand::Logout => {
            auth::clear_token()?;
            println!(
                "{}",
                serde_json::json!({
                    "status": "logged_out",
                    "message": "Credentials cleared"
                })
            );
            Ok(())
        }

        AuthCommand::Status => {
            let authenticated = auth::has_token();
            let storage = auth::token_storage_type();
            let cred_type = auth::credential_type();
            println!(
                "{}",
                serde_json::json!({
                    "authenticated": authenticated,
                    "type": cred_type,
                    "storage": storage,
                })
            );
            Ok(())
        }

        AuthCommand::Token => {
            let token = auth::resolve_token(None)?;
            print!("{token}");
            Ok(())
        }
    }
}

fn login_pat(token: Option<&str>) -> Result<()> {
    let token_value = match token {
        Some(t) => t.to_string(),
        None => {
            eprint!("Enter your YNAB personal access token: ");
            std::io::stdout().flush()?;
            let mut input = String::new();
            std::io::stdin().read_line(&mut input)?;
            input.trim().to_string()
        }
    };

    if token_value.is_empty() {
        anyhow::bail!("Token cannot be empty");
    }

    auth::store_token(&token_value)?;
    let storage = auth::token_storage_type();
    println!(
        "{}",
        serde_json::json!({
            "status": "authenticated",
            "type": "personal_access_token",
            "storage": storage,
            "message": format!("Token stored in {storage}")
        })
    );
    Ok(())
}

async fn login_oauth() -> Result<()> {
    let code_verifier = auth::generate_code_verifier();
    let code_challenge = auth::compute_code_challenge(&code_verifier);
    let state = auth::generate_state();

    // Build authorization URL
    let mut auth_url = Url::parse(auth::OAUTH_AUTHORIZE_URL)?;
    auth_url.query_pairs_mut().extend_pairs(&[
        ("client_id", auth::OAUTH_CLIENT_ID),
        ("redirect_uri", auth::OAUTH_REDIRECT_URI),
        ("response_type", "code"),
        ("code_challenge", &code_challenge),
        ("code_challenge_method", "S256"),
        ("state", &state),
    ]);

    // Start local server before opening browser
    let listener = TcpListener::bind("127.0.0.1:14512")
        .map_err(|e| anyhow::anyhow!("Failed to start local server on port 14512: {e}"))?;

    eprintln!("Opening browser for YNAB authorization...");
    eprintln!("If the browser doesn't open, visit:\n  {auth_url}\n");

    if open::that(auth_url.as_str()).is_err() {
        eprintln!("Could not open browser automatically.");
    }

    // Wait for the callback
    let (code, received_state) = wait_for_callback(&listener)?;

    // Validate state
    if received_state != state {
        anyhow::bail!("OAuth state mismatch — possible CSRF attack. Try again.");
    }

    eprintln!("Authorization received, exchanging code for tokens...");

    // Exchange code for tokens
    let token_resp = auth::exchange_code(&code, &code_verifier).await?;

    let refresh_token = token_resp.refresh_token.as_deref().unwrap_or_default();
    let expires_in = token_resp.expires_in.unwrap_or(7200);

    auth::store_oauth_tokens(&token_resp.access_token, refresh_token, expires_in)?;

    let storage = auth::token_storage_type();
    println!(
        "{}",
        serde_json::json!({
            "status": "authenticated",
            "type": "oauth",
            "storage": storage,
            "message": format!("OAuth tokens stored in {storage}")
        })
    );
    Ok(())
}

/// Wait for the OAuth callback on the local server. Returns (code, state).
fn wait_for_callback(listener: &TcpListener) -> Result<(String, String)> {
    let (mut stream, _) = listener
        .accept()
        .map_err(|e| anyhow::anyhow!("Failed to accept connection: {e}"))?;

    let mut reader = BufReader::new(&stream);
    let mut request_line = String::new();
    reader.read_line(&mut request_line)?;

    // Parse the GET request to extract query parameters
    // Format: GET /callback?code=...&state=... HTTP/1.1
    let path = request_line
        .split_whitespace()
        .nth(1)
        .ok_or_else(|| anyhow::anyhow!("Invalid HTTP request"))?;

    let url = Url::parse(&format!("http://localhost{path}"))?;
    let params: std::collections::HashMap<String, String> = url
        .query_pairs()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect();

    // Check for error response
    if let Some(error) = params.get("error") {
        let desc = params
            .get("error_description")
            .map(|d| format!(": {d}"))
            .unwrap_or_default();
        // Send error page
        let body = format!(
            "<html><body><h2>Authorization failed</h2><p>{error}{desc}</p><p>You can close this tab.</p></body></html>"
        );
        let response = format!(
            "HTTP/1.1 400 Bad Request\r\nContent-Type: text/html\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
            body.len(),
            body
        );
        let _ = stream.write_all(response.as_bytes());
        anyhow::bail!("Authorization denied: {error}{desc}");
    }

    let code = params
        .get("code")
        .ok_or_else(|| anyhow::anyhow!("No authorization code in callback"))?
        .clone();

    let received_state = params
        .get("state")
        .ok_or_else(|| anyhow::anyhow!("No state parameter in callback"))?
        .clone();

    // Send success page
    let body = "<html><body><h2>Authorization successful!</h2><p>You can close this tab and return to the terminal.</p></body></html>";
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
        body.len(),
        body
    );
    let _ = stream.write_all(response.as_bytes());

    Ok((code, received_state))
}
