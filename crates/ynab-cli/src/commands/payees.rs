use anyhow::Result;
use ynab_client::YnabClient;

use crate::cli::{OutputFormat, PayeesCommand};
use crate::commands::plans::resolve_plan_id;
use crate::output;

pub async fn run(
    client: &YnabClient,
    command: &PayeesCommand,
    format: &OutputFormat,
    plan_id: Option<&str>,
    dry_run: bool,
) -> Result<()> {
    let plan_id = resolve_plan_id(None, plan_id)?;

    match command {
        PayeesCommand::List { last_knowledge } => {
            if dry_run {
                output::output(
                    &client.dry_run_request("GET", &format!("/plans/{plan_id}/payees"), None),
                    format,
                )?;
                return Ok(());
            }
            let data = client.get_payees(&plan_id, *last_knowledge).await?;
            output::output(&data, format)?;
        }

        PayeesCommand::Get { payee_id } => {
            if dry_run {
                output::output(
                    &client.dry_run_request(
                        "GET",
                        &format!("/plans/{plan_id}/payees/{payee_id}"),
                        None,
                    ),
                    format,
                )?;
                return Ok(());
            }
            let payee = client.get_payee(&plan_id, payee_id).await?;
            output::output(&payee, format)?;
        }
    }
    Ok(())
}
