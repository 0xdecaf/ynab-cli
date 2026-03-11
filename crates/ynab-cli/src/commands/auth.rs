use anyhow::Result;
use ynab_client::auth;

use crate::cli::AuthCommand;

pub fn run(command: &AuthCommand) -> Result<()> {
    match command {
        AuthCommand::Login { pat, token } => {
            if !pat {
                anyhow::bail!(
                    "Currently only --pat (personal access token) login is supported.\n\
                    Usage: ynab auth login --pat [--token <TOKEN>]"
                );
            }

            let token_value = match token {
                Some(t) => t.clone(),
                None => {
                    eprint!("Enter your YNAB personal access token: ");
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
            println!(
                "{}",
                serde_json::json!({
                    "authenticated": authenticated,
                    "type": if authenticated { "personal_access_token" } else { "none" },
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
