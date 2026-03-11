use anyhow::Result;
use ynab_client::YnabClient;

use crate::cli::UserCommand;
use crate::output::{self, OutputConfig};

pub async fn run(
    client: &YnabClient,
    command: &UserCommand,
    out: &OutputConfig<'_>,
    dry_run: bool,
) -> Result<()> {
    match command {
        UserCommand::Get => {
            if dry_run {
                output::output(&client.dry_run_request("GET", "/user", None), out)?;
                return Ok(());
            }
            let user = client.get_user().await?;
            output::output(&user, out)?;
        }
    }
    Ok(())
}
