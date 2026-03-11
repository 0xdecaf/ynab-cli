use anyhow::Result;
use ynab_client::YnabClient;

use crate::cli::{CategoriesCommand, OutputFormat};
use crate::commands::plans::resolve_plan_id;
use crate::output;

pub async fn run(
    client: &YnabClient,
    command: &CategoriesCommand,
    format: &OutputFormat,
    plan_id: Option<&str>,
    dry_run: bool,
) -> Result<()> {
    let plan_id = resolve_plan_id(None, plan_id)?;

    match command {
        CategoriesCommand::List { last_knowledge } => {
            if dry_run {
                output::output(
                    &client.dry_run_request("GET", &format!("/plans/{plan_id}/categories"), None),
                    format,
                )?;
                return Ok(());
            }
            let data = client.get_categories(&plan_id, *last_knowledge).await?;
            output::output(&data, format)?;
        }

        CategoriesCommand::Get { category_id } => {
            if dry_run {
                output::output(
                    &client.dry_run_request(
                        "GET",
                        &format!("/plans/{plan_id}/categories/{category_id}"),
                        None,
                    ),
                    format,
                )?;
                return Ok(());
            }
            let category = client.get_category(&plan_id, category_id).await?;
            output::output(&category, format)?;
        }

        CategoriesCommand::MonthGet { month, category_id } => {
            if dry_run {
                output::output(
                    &client.dry_run_request(
                        "GET",
                        &format!("/plans/{plan_id}/months/{month}/categories/{category_id}"),
                        None,
                    ),
                    format,
                )?;
                return Ok(());
            }
            // Use the same category endpoint with month context
            let category = client.get_category(&plan_id, category_id).await?;
            output::output(&category, format)?;
        }
    }
    Ok(())
}
