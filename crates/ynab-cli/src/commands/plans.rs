use anyhow::Result;
use ynab_client::YnabClient;

use crate::cli::PlansCommand;
use crate::output::{self, OutputConfig};

pub async fn run(
    client: &YnabClient,
    command: &PlansCommand,
    out: &OutputConfig<'_>,
    plan_id: Option<&str>,
    dry_run: bool,
) -> Result<()> {
    match command {
        PlansCommand::List => {
            if dry_run {
                output::output(&client.dry_run_request("GET", "/plans", None), out)?;
                return Ok(());
            }
            let data = client.get_plans().await?;
            output::output(&data, out)?;
        }

        PlansCommand::Get { id } => {
            let plan_id = resolve_plan_id(id.as_deref(), plan_id)?;
            if dry_run {
                output::output(
                    &client.dry_run_request("GET", &format!("/plans/{plan_id}"), None),
                    out,
                )?;
                return Ok(());
            }
            let data = client.get_plan(&plan_id).await?;
            output::output(&data, out)?;
        }

        PlansCommand::Settings { id } => {
            let plan_id = resolve_plan_id(id.as_deref(), plan_id)?;
            if dry_run {
                output::output(
                    &client.dry_run_request("GET", &format!("/plans/{plan_id}/settings"), None),
                    out,
                )?;
                return Ok(());
            }
            let settings = client.get_plan_settings(&plan_id).await?;
            output::output(&settings, out)?;
        }

        PlansCommand::SetDefault { id } => {
            let mut config = ynab_client::Config::load()?;
            config.set_default_plan_id(id);
            config.save()?;
            println!("Default plan ID set to: {id}");
        }
    }
    Ok(())
}

pub fn resolve_plan_id(explicit_id: Option<&str>, global_id: Option<&str>) -> Result<String> {
    if let Some(id) = explicit_id.or(global_id) {
        return Ok(id.to_string());
    }

    // Try config file fallback
    if let Ok(config) = ynab_client::Config::load()
        && let Some(default_id) = config.default_plan_id()
    {
        return Ok(default_id.to_string());
    }

    // Fall back to "default" — the YNAB API accepts this as the user's default budget
    Ok("default".to_string())
}
