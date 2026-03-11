use anyhow::Result;

/// Display the schema for a given resource.method pair.
/// Reads from the vendored OpenAPI spec embedded at build time.
pub fn run(resource_method: &str) -> Result<()> {
    // For Phase 1, output a helpful message pointing to the spec.
    // Phase 3 will embed the full OpenAPI spec and lookup schemas dynamically.
    let parts: Vec<&str> = resource_method.split('.').collect();
    if parts.len() != 2 {
        anyhow::bail!(
            "Invalid format. Use: ynab schema <resource>.<method>\n\
             Example: ynab schema transactions.create"
        );
    }

    let resource = parts[0];
    let method = parts[1];

    // Map common resource.method pairs to API info
    let schema_info = get_schema_info(resource, method);

    match schema_info {
        Some(info) => {
            println!("{}", serde_json::to_string_pretty(&info)?);
        }
        None => {
            anyhow::bail!(
                "Unknown resource.method: {resource_method}\n\
                 Available resources: plans, accounts, transactions, categories, \
                 payees, payee-locations, months, scheduled, money-movements, user"
            );
        }
    }

    Ok(())
}

fn get_schema_info(resource: &str, method: &str) -> Option<serde_json::Value> {
    let info = match (resource, method) {
        ("plans", "list") => serde_json::json!({
            "resource": "plans",
            "method": "list",
            "http_method": "GET",
            "path": "/plans",
            "description": "List all plans (budgets). Returns plan ID, name, and format settings.",
            "parameters": {},
            "response": "PlansData { plans: [PlanSummary], default_plan: PlanSummary? }"
        }),
        ("plans", "get") => serde_json::json!({
            "resource": "plans",
            "method": "get",
            "http_method": "GET",
            "path": "/plans/{plan_id}",
            "description": "Get a single plan with full detail including accounts, categories, etc.",
            "parameters": {
                "plan_id": "string (required) - Plan UUID"
            },
            "response": "PlanDetailData { plan: PlanDetail, server_knowledge: i64 }"
        }),
        ("accounts", "list") => serde_json::json!({
            "resource": "accounts",
            "method": "list",
            "http_method": "GET",
            "path": "/plans/{plan_id}/accounts",
            "parameters": {
                "plan_id": "string (required)",
                "last_knowledge_of_server": "i64 (optional) - delta sync"
            },
            "response": "AccountsData { accounts: [Account], server_knowledge: i64 }"
        }),
        ("accounts", "create") => serde_json::json!({
            "resource": "accounts",
            "method": "create",
            "http_method": "POST",
            "path": "/plans/{plan_id}/accounts",
            "parameters": {
                "plan_id": "string (required)"
            },
            "request_body": {
                "account": {
                    "name": "string (required)",
                    "type": "string (required) - checking|savings|cash|creditCard|lineOfCredit|otherAsset|otherLiability|mortgage|autoLoan|studentLoan|personalLoan|medicalDebt|otherDebt",
                    "balance": "integer (required) - milliunits"
                }
            }
        }),
        ("transactions", "list") => serde_json::json!({
            "resource": "transactions",
            "method": "list",
            "http_method": "GET",
            "path": "/plans/{plan_id}/transactions",
            "parameters": {
                "plan_id": "string (required)",
                "since_date": "string (optional) - YYYY-MM-DD",
                "type": "string (optional) - uncategorized|unapproved",
                "last_knowledge_of_server": "i64 (optional) - delta sync"
            },
            "response": "TransactionsData { transactions: [TransactionDetail], server_knowledge: i64 }"
        }),
        ("transactions", "create") => serde_json::json!({
            "resource": "transactions",
            "method": "create",
            "http_method": "POST",
            "path": "/plans/{plan_id}/transactions",
            "parameters": {
                "plan_id": "string (required)"
            },
            "request_body": {
                "transaction": {
                    "account_id": "string/uuid (required)",
                    "date": "string (required) - YYYY-MM-DD",
                    "amount": "integer (required) - milliunits (1000 = $1.00, negative = outflow)",
                    "payee_id": "string/uuid (optional)",
                    "payee_name": "string (optional, max 100 chars)",
                    "category_id": "string/uuid (optional)",
                    "memo": "string (optional, max 200 chars)",
                    "cleared": "string (optional) - cleared|uncleared|reconciled",
                    "approved": "boolean (optional)",
                    "flag_color": "string (optional) - red|orange|yellow|green|blue|purple",
                    "import_id": "string (optional) - for deduplication",
                    "subtransactions": "array (optional) - for split transactions"
                }
            }
        }),
        ("transactions", "update") => serde_json::json!({
            "resource": "transactions",
            "method": "update",
            "http_method": "PUT",
            "path": "/plans/{plan_id}/transactions/{transaction_id}",
            "parameters": {
                "plan_id": "string (required)",
                "transaction_id": "string (required)"
            },
            "request_body": {
                "transaction": "Same fields as create (all optional for update)"
            }
        }),
        ("transactions", "delete") => serde_json::json!({
            "resource": "transactions",
            "method": "delete",
            "http_method": "DELETE",
            "path": "/plans/{plan_id}/transactions/{transaction_id}",
            "parameters": {
                "plan_id": "string (required)",
                "transaction_id": "string (required)"
            }
        }),
        ("categories", "list") => serde_json::json!({
            "resource": "categories",
            "method": "list",
            "http_method": "GET",
            "path": "/plans/{plan_id}/categories",
            "parameters": {
                "plan_id": "string (required)",
                "last_knowledge_of_server": "i64 (optional)"
            },
            "response": "CategoriesData { category_groups: [CategoryGroupWithCategories], server_knowledge: i64 }"
        }),
        ("payees", "list") => serde_json::json!({
            "resource": "payees",
            "method": "list",
            "http_method": "GET",
            "path": "/plans/{plan_id}/payees",
            "parameters": {
                "plan_id": "string (required)",
                "last_knowledge_of_server": "i64 (optional)"
            },
            "response": "PayeesData { payees: [Payee], server_knowledge: i64 }"
        }),
        ("months", "list") => serde_json::json!({
            "resource": "months",
            "method": "list",
            "http_method": "GET",
            "path": "/plans/{plan_id}/months",
            "parameters": {
                "plan_id": "string (required)",
                "last_knowledge_of_server": "i64 (optional)"
            },
            "response": "MonthsData { months: [MonthSummary], server_knowledge: i64 }"
        }),
        ("months", "get") => serde_json::json!({
            "resource": "months",
            "method": "get",
            "http_method": "GET",
            "path": "/plans/{plan_id}/months/{month}",
            "parameters": {
                "plan_id": "string (required)",
                "month": "string (required) - YYYY-MM-DD"
            },
            "response": "MonthDetailData { month: MonthDetail }"
        }),
        ("scheduled", "list") => serde_json::json!({
            "resource": "scheduled",
            "method": "list",
            "http_method": "GET",
            "path": "/plans/{plan_id}/scheduled_transactions",
            "parameters": {
                "plan_id": "string (required)",
                "last_knowledge_of_server": "i64 (optional)"
            },
            "response": "ScheduledTransactionsData { scheduled_transactions: [ScheduledTransactionDetail], server_knowledge: i64 }"
        }),
        ("user", "get") => serde_json::json!({
            "resource": "user",
            "method": "get",
            "http_method": "GET",
            "path": "/user",
            "parameters": {},
            "response": "UserData { user: User { id: string } }"
        }),
        _ => return None,
    };

    Some(info)
}
