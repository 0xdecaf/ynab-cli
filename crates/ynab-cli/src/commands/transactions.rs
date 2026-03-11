use anyhow::Result;
use ynab_client::YnabClient;
use ynab_types::SaveTransaction;

use crate::cli::{OutputFormat, TransactionsCommand};
use crate::commands::plans::resolve_plan_id;
use crate::output;

pub async fn run(
    client: &YnabClient,
    command: &TransactionsCommand,
    format: &OutputFormat,
    plan_id: Option<&str>,
    dry_run: bool,
) -> Result<()> {
    let plan_id = resolve_plan_id(None, plan_id)?;

    match command {
        TransactionsCommand::List {
            since_date,
            r#type,
            last_knowledge,
        } => {
            if dry_run {
                output::output(
                    &client.dry_run_request(
                        "GET",
                        &format!("/plans/{plan_id}/transactions"),
                        None,
                    ),
                    format,
                )?;
                return Ok(());
            }
            let data = client
                .get_transactions(
                    &plan_id,
                    since_date.as_deref(),
                    r#type.as_deref(),
                    *last_knowledge,
                )
                .await?;
            output::output(&data, format)?;
        }

        TransactionsCommand::Get { transaction_id } => {
            if dry_run {
                output::output(
                    &client.dry_run_request(
                        "GET",
                        &format!("/plans/{plan_id}/transactions/{transaction_id}"),
                        None,
                    ),
                    format,
                )?;
                return Ok(());
            }
            let txn = client.get_transaction(&plan_id, transaction_id).await?;
            output::output(&txn, format)?;
        }

        TransactionsCommand::Create { json } => {
            let transaction: SaveTransaction = serde_json::from_str(json)?;
            if dry_run {
                let body = serde_json::json!({ "transaction": transaction });
                output::output(
                    &client.dry_run_request(
                        "POST",
                        &format!("/plans/{plan_id}/transactions"),
                        Some(&body),
                    ),
                    format,
                )?;
                return Ok(());
            }
            let data = client.create_transaction(&plan_id, &transaction).await?;
            output::output(&data, format)?;
        }

        TransactionsCommand::Update {
            transaction_id,
            json,
        } => {
            let transaction: serde_json::Value = serde_json::from_str(json)?;
            if dry_run {
                let body = serde_json::json!({ "transaction": transaction });
                output::output(
                    &client.dry_run_request(
                        "PUT",
                        &format!("/plans/{plan_id}/transactions/{transaction_id}"),
                        Some(&body),
                    ),
                    format,
                )?;
                return Ok(());
            }
            let txn = client
                .update_transaction(&plan_id, transaction_id, &transaction)
                .await?;
            output::output(&txn, format)?;
        }

        TransactionsCommand::UpdateBulk { json } => {
            let transactions: Vec<serde_json::Value> = serde_json::from_str(json)?;
            if dry_run {
                let body = serde_json::json!({ "transactions": transactions });
                output::output(
                    &client.dry_run_request(
                        "PATCH",
                        &format!("/plans/{plan_id}/transactions"),
                        Some(&body),
                    ),
                    format,
                )?;
                return Ok(());
            }
            let data = client
                .update_transactions_bulk(&plan_id, &transactions)
                .await?;
            output::output(&data, format)?;
        }

        TransactionsCommand::Delete { transaction_id } => {
            if dry_run {
                output::output(
                    &client.dry_run_request(
                        "DELETE",
                        &format!("/plans/{plan_id}/transactions/{transaction_id}"),
                        None,
                    ),
                    format,
                )?;
                return Ok(());
            }
            let txn = client.delete_transaction(&plan_id, transaction_id).await?;
            output::output(&txn, format)?;
        }

        TransactionsCommand::Import => {
            if dry_run {
                output::output(
                    &client.dry_run_request(
                        "POST",
                        &format!("/plans/{plan_id}/transactions/import"),
                        None,
                    ),
                    format,
                )?;
                return Ok(());
            }
            let data = client.import_transactions(&plan_id).await?;
            output::output(&data, format)?;
        }

        TransactionsCommand::ByAccount {
            account_id,
            since_date,
            last_knowledge,
        } => {
            if dry_run {
                output::output(
                    &client.dry_run_request(
                        "GET",
                        &format!("/plans/{plan_id}/accounts/{account_id}/transactions"),
                        None,
                    ),
                    format,
                )?;
                return Ok(());
            }
            let data = client
                .get_transactions_by_account(
                    &plan_id,
                    account_id,
                    since_date.as_deref(),
                    *last_knowledge,
                )
                .await?;
            output::output(&data, format)?;
        }

        TransactionsCommand::ByCategory {
            category_id,
            since_date,
            last_knowledge,
        } => {
            if dry_run {
                output::output(
                    &client.dry_run_request(
                        "GET",
                        &format!("/plans/{plan_id}/categories/{category_id}/transactions"),
                        None,
                    ),
                    format,
                )?;
                return Ok(());
            }
            let data = client
                .get_transactions_by_category(
                    &plan_id,
                    category_id,
                    since_date.as_deref(),
                    *last_knowledge,
                )
                .await?;
            output::output(&data, format)?;
        }

        TransactionsCommand::ByPayee {
            payee_id,
            since_date,
            last_knowledge,
        } => {
            if dry_run {
                output::output(
                    &client.dry_run_request(
                        "GET",
                        &format!("/plans/{plan_id}/payees/{payee_id}/transactions"),
                        None,
                    ),
                    format,
                )?;
                return Ok(());
            }
            let data = client
                .get_transactions_by_payee(
                    &plan_id,
                    payee_id,
                    since_date.as_deref(),
                    *last_knowledge,
                )
                .await?;
            output::output(&data, format)?;
        }

        TransactionsCommand::ByMonth {
            month,
            last_knowledge,
        } => {
            if dry_run {
                output::output(
                    &client.dry_run_request(
                        "GET",
                        &format!("/plans/{plan_id}/months/{month}/transactions"),
                        None,
                    ),
                    format,
                )?;
                return Ok(());
            }
            let data = client
                .get_transactions_by_month(&plan_id, month, *last_knowledge)
                .await?;
            output::output(&data, format)?;
        }
    }
    Ok(())
}
