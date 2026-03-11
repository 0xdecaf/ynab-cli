use anyhow::Result;
use ynab_client::YnabClient;

use crate::cli::{OutputFormat, PayeeLocationsCommand};
use crate::commands::plans::resolve_plan_id;
use crate::output;

pub async fn run(
    client: &YnabClient,
    command: &PayeeLocationsCommand,
    format: &OutputFormat,
    plan_id: Option<&str>,
    dry_run: bool,
) -> Result<()> {
    let plan_id = resolve_plan_id(None, plan_id)?;

    match command {
        PayeeLocationsCommand::List => {
            if dry_run {
                output::output(
                    &client.dry_run_request(
                        "GET",
                        &format!("/plans/{plan_id}/payee_locations"),
                        None,
                    ),
                    format,
                )?;
                return Ok(());
            }
            let locations = client.get_payee_locations(&plan_id).await?;
            output::output(&locations, format)?;
        }

        PayeeLocationsCommand::Get { payee_location_id } => {
            if dry_run {
                output::output(
                    &client.dry_run_request(
                        "GET",
                        &format!("/plans/{plan_id}/payee_locations/{payee_location_id}"),
                        None,
                    ),
                    format,
                )?;
                return Ok(());
            }
            let location = client
                .get_payee_location(&plan_id, payee_location_id)
                .await?;
            output::output(&location, format)?;
        }

        PayeeLocationsCommand::ByPayee { payee_id } => {
            if dry_run {
                output::output(
                    &client.dry_run_request(
                        "GET",
                        &format!("/plans/{plan_id}/payees/{payee_id}/payee_locations"),
                        None,
                    ),
                    format,
                )?;
                return Ok(());
            }
            let locations = client
                .get_payee_locations_by_payee(&plan_id, payee_id)
                .await?;
            output::output(&locations, format)?;
        }
    }
    Ok(())
}
