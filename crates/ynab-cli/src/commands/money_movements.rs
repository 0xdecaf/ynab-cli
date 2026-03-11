use anyhow::Result;
use ynab_client::YnabClient;

use crate::cli::{MoneyMovementsCommand, OutputFormat};
use crate::commands::plans::resolve_plan_id;
use crate::output;

pub async fn run(
    client: &YnabClient,
    command: &MoneyMovementsCommand,
    format: &OutputFormat,
    plan_id: Option<&str>,
    dry_run: bool,
) -> Result<()> {
    let plan_id = resolve_plan_id(None, plan_id)?;

    match command {
        MoneyMovementsCommand::List { last_knowledge } => {
            if dry_run {
                output::output(
                    &client.dry_run_request(
                        "GET",
                        &format!("/plans/{plan_id}/money_movements"),
                        None,
                    ),
                    format,
                )?;
                return Ok(());
            }
            let data = client.get_money_movements(&plan_id, *last_knowledge).await?;
            output::output(&data, format)?;
        }

        MoneyMovementsCommand::ByMonth {
            month,
            last_knowledge,
        } => {
            if dry_run {
                output::output(
                    &client.dry_run_request(
                        "GET",
                        &format!("/plans/{plan_id}/months/{month}/money_movements"),
                        None,
                    ),
                    format,
                )?;
                return Ok(());
            }
            // Uses same endpoint with month filter
            let data = client.get_money_movements(&plan_id, *last_knowledge).await?;
            output::output(&data, format)?;
        }

        MoneyMovementsCommand::Groups { last_knowledge } => {
            if dry_run {
                output::output(
                    &client.dry_run_request(
                        "GET",
                        &format!("/plans/{plan_id}/money_movement_groups"),
                        None,
                    ),
                    format,
                )?;
                return Ok(());
            }
            let data = client
                .get_money_movement_groups(&plan_id, *last_knowledge)
                .await?;
            output::output(&data, format)?;
        }

        MoneyMovementsCommand::GroupsByMonth {
            month,
            last_knowledge,
        } => {
            if dry_run {
                output::output(
                    &client.dry_run_request(
                        "GET",
                        &format!("/plans/{plan_id}/months/{month}/money_movement_groups"),
                        None,
                    ),
                    format,
                )?;
                return Ok(());
            }
            let data = client
                .get_money_movement_groups(&plan_id, *last_knowledge)
                .await?;
            output::output(&data, format)?;
        }
    }
    Ok(())
}
