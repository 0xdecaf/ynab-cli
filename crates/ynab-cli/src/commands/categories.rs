use anyhow::Result;
use ynab_client::YnabClient;

use crate::cli::CategoriesCommand;
use crate::commands::plans::resolve_plan_id;
use crate::output::{self, OutputConfig};

pub async fn run(
    client: &YnabClient,
    command: &CategoriesCommand,
    out: &OutputConfig<'_>,
    plan_id: Option<&str>,
    dry_run: bool,
) -> Result<()> {
    let plan_id = resolve_plan_id(None, plan_id)?;

    match command {
        CategoriesCommand::List { last_knowledge } => {
            if dry_run {
                output::output(
                    &client.dry_run_request("GET", &format!("/plans/{plan_id}/categories"), None),
                    out,
                )?;
                return Ok(());
            }
            let data = client.get_categories(&plan_id, *last_knowledge).await?;
            output::output(&data, out)?;
        }

        CategoriesCommand::Get { category_id } => {
            if dry_run {
                output::output(
                    &client.dry_run_request(
                        "GET",
                        &format!("/plans/{plan_id}/categories/{category_id}"),
                        None,
                    ),
                    out,
                )?;
                return Ok(());
            }
            let category = client.get_category(&plan_id, category_id).await?;
            output::output(&category, out)?;
        }

        CategoriesCommand::MonthGet { month, category_id } => {
            if dry_run {
                output::output(
                    &client.dry_run_request(
                        "GET",
                        &format!("/plans/{plan_id}/months/{month}/categories/{category_id}"),
                        None,
                    ),
                    out,
                )?;
                return Ok(());
            }
            let category = client
                .get_month_category(&plan_id, month, category_id)
                .await?;
            output::output(&category, out)?;
        }

        CategoriesCommand::Update { category_id, json } => {
            let category: serde_json::Value = serde_json::from_str(json)?;
            if dry_run {
                let body = serde_json::json!({ "category": category });
                output::output(
                    &client.dry_run_request(
                        "PATCH",
                        &format!("/plans/{plan_id}/categories/{category_id}"),
                        Some(&body),
                    ),
                    out,
                )?;
                return Ok(());
            }
            let result = client
                .update_category(&plan_id, category_id, &category)
                .await?;
            output::output(&result, out)?;
        }

        CategoriesCommand::Budget {
            month,
            category_id,
            budgeted,
        } => {
            if dry_run {
                let body = serde_json::json!({ "category": { "budgeted": budgeted } });
                output::output(
                    &client.dry_run_request(
                        "PATCH",
                        &format!("/plans/{plan_id}/months/{month}/categories/{category_id}"),
                        Some(&body),
                    ),
                    out,
                )?;
                return Ok(());
            }
            let result = client
                .update_category_month(&plan_id, month, category_id, *budgeted)
                .await?;
            output::output(&result, out)?;
        }
    }
    Ok(())
}
