use anyhow::Result;
use ynab_client::YnabClient;

use crate::cli::{OutputFormat, UserCommand};
use crate::output;

pub async fn run(
    client: &YnabClient,
    command: &UserCommand,
    format: &OutputFormat,
    dry_run: bool,
) -> Result<()> {
    match command {
        UserCommand::Get => {
            if dry_run {
                output::output(&client.dry_run_request("GET", "/user", None), format)?;
                return Ok(());
            }
            let user = client.get_user().await?;
            output::output(&user, format)?;
        }
    }
    Ok(())
}
