use anyhow::Result;
use ynab_client::YnabClient;

use crate::cli::ScheduledCommand;
use crate::commands::plans::resolve_plan_id;
use crate::output::{self, OutputConfig};

pub async fn run(
    client: &YnabClient,
    command: &ScheduledCommand,
    out: &OutputConfig<'_>,
    plan_id: Option<&str>,
    dry_run: bool,
) -> Result<()> {
    let plan_id = resolve_plan_id(None, plan_id)?;

    match command {
        ScheduledCommand::List { last_knowledge } => {
            if dry_run {
                output::output(
                    &client.dry_run_request(
                        "GET",
                        &format!("/plans/{plan_id}/scheduled_transactions"),
                        None,
                    ),
                    out,
                )?;
                return Ok(());
            }
            let data = client
                .get_scheduled_transactions(&plan_id, *last_knowledge)
                .await?;
            output::output(&data, out)?;
        }

        ScheduledCommand::Get {
            scheduled_transaction_id,
        } => {
            if dry_run {
                output::output(
                    &client.dry_run_request(
                        "GET",
                        &format!(
                            "/plans/{plan_id}/scheduled_transactions/{scheduled_transaction_id}"
                        ),
                        None,
                    ),
                    out,
                )?;
                return Ok(());
            }
            let txn = client
                .get_scheduled_transaction(&plan_id, scheduled_transaction_id)
                .await?;
            output::output(&txn, out)?;
        }

        ScheduledCommand::Delete {
            scheduled_transaction_id,
        } => {
            if dry_run {
                output::output(
                    &client.dry_run_request(
                        "DELETE",
                        &format!(
                            "/plans/{plan_id}/scheduled_transactions/{scheduled_transaction_id}"
                        ),
                        None,
                    ),
                    out,
                )?;
                return Ok(());
            }
            let txn = client
                .delete_scheduled_transaction(&plan_id, scheduled_transaction_id)
                .await?;
            output::output(&txn, out)?;
        }
    }
    Ok(())
}
