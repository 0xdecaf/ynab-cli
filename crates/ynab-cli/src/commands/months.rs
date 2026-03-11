use anyhow::Result;
use ynab_client::YnabClient;

use crate::cli::{MonthsCommand, OutputFormat};
use crate::commands::plans::resolve_plan_id;
use crate::output;

pub async fn run(
    client: &YnabClient,
    command: &MonthsCommand,
    format: &OutputFormat,
    plan_id: Option<&str>,
    dry_run: bool,
) -> Result<()> {
    let plan_id = resolve_plan_id(None, plan_id)?;

    match command {
        MonthsCommand::List { last_knowledge } => {
            if dry_run {
                output::output(
                    &client.dry_run_request("GET", &format!("/plans/{plan_id}/months"), None),
                    format,
                )?;
                return Ok(());
            }
            let data = client.get_months(&plan_id, *last_knowledge).await?;
            output::output(&data, format)?;
        }

        MonthsCommand::Get { month } => {
            if dry_run {
                output::output(
                    &client.dry_run_request(
                        "GET",
                        &format!("/plans/{plan_id}/months/{month}"),
                        None,
                    ),
                    format,
                )?;
                return Ok(());
            }
            let detail = client.get_month(&plan_id, month).await?;
            output::output(&detail, format)?;
        }
    }
    Ok(())
}
