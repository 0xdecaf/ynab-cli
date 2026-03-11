use anyhow::Result;
use ynab_client::YnabClient;

use crate::cli::{OutputFormat, PlansCommand};
use crate::output;

pub async fn run(
    client: &YnabClient,
    command: &PlansCommand,
    format: &OutputFormat,
    plan_id: Option<&str>,
    dry_run: bool,
) -> Result<()> {
    match command {
        PlansCommand::List => {
            if dry_run {
                output::output(&client.dry_run_request("GET", "/plans", None), format)?;
                return Ok(());
            }
            let data = client.get_plans().await?;
            output::output(&data, format)?;
        }

        PlansCommand::Get { id } => {
            let plan_id = resolve_plan_id(id.as_deref(), plan_id)?;
            if dry_run {
                output::output(
                    &client.dry_run_request("GET", &format!("/plans/{plan_id}"), None),
                    format,
                )?;
                return Ok(());
            }
            let data = client.get_plan(&plan_id).await?;
            output::output(&data, format)?;
        }

        PlansCommand::Settings { id } => {
            let plan_id = resolve_plan_id(id.as_deref(), plan_id)?;
            if dry_run {
                output::output(
                    &client.dry_run_request("GET", &format!("/plans/{plan_id}/settings"), None),
                    format,
                )?;
                return Ok(());
            }
            let settings = client.get_plan_settings(&plan_id).await?;
            output::output(&settings, format)?;
        }
    }
    Ok(())
}

pub fn resolve_plan_id(explicit_id: Option<&str>, global_id: Option<&str>) -> Result<String> {
    explicit_id
        .or(global_id)
        .map(|s| s.to_string())
        .ok_or_else(|| {
            anyhow::anyhow!(
                "Plan ID is required. Provide --plan-id or set YNAB_PLAN_ID.\n\
                 Run `ynab plans list` to see available plans."
            )
        })
}
