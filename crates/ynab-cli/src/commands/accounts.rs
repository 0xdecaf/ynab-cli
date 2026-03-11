use anyhow::Result;
use ynab_client::YnabClient;
use ynab_types::SaveAccount;

use crate::cli::{AccountsCommand, OutputFormat};
use crate::commands::plans::resolve_plan_id;
use crate::output;

pub async fn run(
    client: &YnabClient,
    command: &AccountsCommand,
    format: &OutputFormat,
    plan_id: Option<&str>,
    dry_run: bool,
) -> Result<()> {
    let plan_id = resolve_plan_id(None, plan_id)?;

    match command {
        AccountsCommand::List { last_knowledge } => {
            if dry_run {
                output::output(
                    &client.dry_run_request("GET", &format!("/plans/{plan_id}/accounts"), None),
                    format,
                )?;
                return Ok(());
            }
            let data = client.get_accounts(&plan_id, *last_knowledge).await?;
            output::output(&data, format)?;
        }

        AccountsCommand::Get { account_id } => {
            if dry_run {
                output::output(
                    &client.dry_run_request(
                        "GET",
                        &format!("/plans/{plan_id}/accounts/{account_id}"),
                        None,
                    ),
                    format,
                )?;
                return Ok(());
            }
            let account = client.get_account(&plan_id, account_id).await?;
            output::output(&account, format)?;
        }

        AccountsCommand::Create { json } => {
            let account: SaveAccount = serde_json::from_str(json)?;
            if dry_run {
                let body = serde_json::json!({ "account": account });
                output::output(
                    &client.dry_run_request(
                        "POST",
                        &format!("/plans/{plan_id}/accounts"),
                        Some(&body),
                    ),
                    format,
                )?;
                return Ok(());
            }
            let created = client.create_account(&plan_id, &account).await?;
            output::output(&created, format)?;
        }
    }
    Ok(())
}
