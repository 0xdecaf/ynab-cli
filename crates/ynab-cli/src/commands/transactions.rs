use anyhow::Result;
use ynab_client::YnabClient;
use ynab_types::SaveTransaction;

use crate::cli::TransactionsCommand;
use crate::commands::plans::resolve_plan_id;
use crate::output::{self, OutputConfig};

pub async fn run(
    client: &YnabClient,
    command: &TransactionsCommand,
    out: &OutputConfig<'_>,
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
                    &client.dry_run_request("GET", &format!("/plans/{plan_id}/transactions"), None),
                    out,
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
            output::output(&data, out)?;
        }

        TransactionsCommand::Get { transaction_id } => {
            if dry_run {
                output::output(
                    &client.dry_run_request(
                        "GET",
                        &format!("/plans/{plan_id}/transactions/{transaction_id}"),
                        None,
                    ),
                    out,
                )?;
                return Ok(());
            }
            let txn = client.get_transaction(&plan_id, transaction_id).await?;
            output::output(&txn, out)?;
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
                    out,
                )?;
                return Ok(());
            }
            let data = client.create_transaction(&plan_id, &transaction).await?;
            output::output(&data, out)?;
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
                    out,
                )?;
                return Ok(());
            }
            let txn = client
                .update_transaction(&plan_id, transaction_id, &transaction)
                .await?;
            output::output(&txn, out)?;
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
                    out,
                )?;
                return Ok(());
            }
            let data = client
                .update_transactions_bulk(&plan_id, &transactions)
                .await?;
            output::output(&data, out)?;
        }

        TransactionsCommand::Delete { transaction_id } => {
            if dry_run {
                output::output(
                    &client.dry_run_request(
                        "DELETE",
                        &format!("/plans/{plan_id}/transactions/{transaction_id}"),
                        None,
                    ),
                    out,
                )?;
                return Ok(());
            }
            let txn = client.delete_transaction(&plan_id, transaction_id).await?;
            output::output(&txn, out)?;
        }

        TransactionsCommand::Import => {
            if dry_run {
                output::output(
                    &client.dry_run_request(
                        "POST",
                        &format!("/plans/{plan_id}/transactions/import"),
                        None,
                    ),
                    out,
                )?;
                return Ok(());
            }
            let data = client.import_transactions(&plan_id).await?;
            output::output(&data, out)?;
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
                    out,
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
            output::output(&data, out)?;
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
                    out,
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
            output::output(&data, out)?;
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
                    out,
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
            output::output(&data, out)?;
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
                    out,
                )?;
                return Ok(());
            }
            let data = client
                .get_transactions_by_month(&plan_id, month, *last_knowledge)
                .await?;
            output::output(&data, out)?;
        }

        TransactionsCommand::Search {
            memo,
            payee_name,
            since_date,
            max_amount,
            min_amount,
        } => {
            if dry_run {
                output::output(
                    &client.dry_run_request("GET", &format!("/plans/{plan_id}/transactions"), None),
                    out,
                )?;
                return Ok(());
            }
            // Fetch all transactions (with optional since_date filter)
            let data = client
                .get_transactions(&plan_id, since_date.as_deref(), None, None)
                .await?;

            // Client-side filtering
            let filtered: Vec<_> = data
                .transactions
                .into_iter()
                .filter(|txn| {
                    if let Some(memo_search) = memo {
                        let memo_lower = memo_search.to_lowercase();
                        if !txn
                            .memo
                            .as_ref()
                            .is_some_and(|m| m.to_lowercase().contains(&memo_lower))
                        {
                            return false;
                        }
                    }
                    if let Some(payee_search) = payee_name {
                        let payee_lower = payee_search.to_lowercase();
                        if !txn
                            .payee_name
                            .as_ref()
                            .is_some_and(|p| p.to_lowercase().contains(&payee_lower))
                        {
                            return false;
                        }
                    }
                    if let Some(max) = max_amount
                        && txn.amount > *max
                    {
                        return false;
                    }
                    if let Some(min) = min_amount
                        && txn.amount < *min
                    {
                        return false;
                    }
                    true
                })
                .collect();

            let result = serde_json::json!({
                "transactions": filtered,
                "count": filtered.len(),
            });
            output::output(&result, out)?;
        }
    }
    Ok(())
}
