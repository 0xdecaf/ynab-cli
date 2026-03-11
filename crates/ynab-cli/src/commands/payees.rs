use anyhow::Result;
use ynab_client::YnabClient;

use crate::cli::PayeesCommand;
use crate::commands::plans::resolve_plan_id;
use crate::output::{self, OutputConfig};

pub async fn run(
    client: &YnabClient,
    command: &PayeesCommand,
    out: &OutputConfig<'_>,
    plan_id: Option<&str>,
    dry_run: bool,
) -> Result<()> {
    let plan_id = resolve_plan_id(None, plan_id)?;

    match command {
        PayeesCommand::List { last_knowledge } => {
            if dry_run {
                output::output(
                    &client.dry_run_request("GET", &format!("/plans/{plan_id}/payees"), None),
                    out,
                )?;
                return Ok(());
            }
            let data = client.get_payees(&plan_id, *last_knowledge).await?;
            output::output(&data, out)?;
        }

        PayeesCommand::Get { payee_id } => {
            if dry_run {
                output::output(
                    &client.dry_run_request(
                        "GET",
                        &format!("/plans/{plan_id}/payees/{payee_id}"),
                        None,
                    ),
                    out,
                )?;
                return Ok(());
            }
            let payee = client.get_payee(&plan_id, payee_id).await?;
            output::output(&payee, out)?;
        }

        PayeesCommand::Update { payee_id, json } => {
            let payee: serde_json::Value = serde_json::from_str(json)?;
            if dry_run {
                let body = serde_json::json!({ "payee": payee });
                output::output(
                    &client.dry_run_request(
                        "PATCH",
                        &format!("/plans/{plan_id}/payees/{payee_id}"),
                        Some(&body),
                    ),
                    out,
                )?;
                return Ok(());
            }
            let result = client.update_payee(&plan_id, payee_id, &payee).await?;
            output::output(&result, out)?;
        }
    }
    Ok(())
}
