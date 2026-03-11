use anyhow::Result;
use ynab_client::YnabClient;

use crate::cli::PayeeLocationsCommand;
use crate::commands::plans::resolve_plan_id;
use crate::output::{self, OutputConfig};

pub async fn run(
    client: &YnabClient,
    command: &PayeeLocationsCommand,
    out: &OutputConfig<'_>,
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
                    out,
                )?;
                return Ok(());
            }
            let data = client.get_payee_locations(&plan_id).await?;
            output::output(&data, out)?;
        }

        PayeeLocationsCommand::Get { payee_location_id } => {
            if dry_run {
                output::output(
                    &client.dry_run_request(
                        "GET",
                        &format!("/plans/{plan_id}/payee_locations/{payee_location_id}"),
                        None,
                    ),
                    out,
                )?;
                return Ok(());
            }
            let location = client
                .get_payee_location(&plan_id, payee_location_id)
                .await?;
            output::output(&location, out)?;
        }

        PayeeLocationsCommand::ByPayee { payee_id } => {
            if dry_run {
                output::output(
                    &client.dry_run_request(
                        "GET",
                        &format!("/plans/{plan_id}/payees/{payee_id}/payee_locations"),
                        None,
                    ),
                    out,
                )?;
                return Ok(());
            }
            let data = client
                .get_payee_locations_by_payee(&plan_id, payee_id)
                .await?;
            output::output(&data, out)?;
        }
    }
    Ok(())
}
