use rmcp::model::{ServerCapabilities, ServerInfo};
use rmcp::{tool, ServerHandler};
use ynab_client::YnabClient;

fn to_json(value: &impl serde::Serialize) -> String {
    serde_json::to_string_pretty(value).unwrap_or_default()
}

const INSTRUCTIONS: &str = "\
YNAB (You Need a Budget) API Server.

All monetary amounts are in milliunits format (divide by 1000 for currency units):
- $25.00 = 25000 milliunits
- -$12.50 = -12500 milliunits (negative = outflow)

Getting started:
1. Call ynab_plans_list to find your plan (budget) IDs
2. Use the plan_id in subsequent calls

Most tools require a plan_id parameter. Get one from ynab_plans_list first.

Delta sync: Many list tools accept last_knowledge_of_server for incremental updates.
Pass the server_knowledge value from a previous response to get only changes since then.";

/// MCP server for the YNAB API.
#[derive(Clone)]
pub struct YnabMcpServer {
    client: YnabClient,
}

impl YnabMcpServer {
    pub fn new(client: YnabClient) -> Self {
        Self { client }
    }

    pub async fn serve_stdio(self) -> anyhow::Result<()> {
        use rmcp::ServiceExt;
        let transport = rmcp::transport::stdio();
        let server = self.serve(transport).await?;
        server.waiting().await?;
        Ok(())
    }
}

#[tool(tool_box)]
impl YnabMcpServer {
    // === User ===

    #[tool(description = "Get the authenticated YNAB user's ID")]
    async fn ynab_user_get(&self) -> Result<String, String> {
        self.client
            .get_user()
            .await
            .map(|u| to_json(&u))
            .map_err(|e| e.to_string())
    }

    // === Plans (Budgets) ===

    #[tool(
        description = "List all YNAB plans (budgets). Returns plan IDs, names, and format settings. Use a plan_id from this response for other tools."
    )]
    async fn ynab_plans_list(&self) -> Result<String, String> {
        self.client
            .get_plans()
            .await
            .map(|d| to_json(&d))
            .map_err(|e| e.to_string())
    }

    #[tool(
        description = "Get a single YNAB plan with full details including accounts, categories, payees, and transactions."
    )]
    async fn ynab_plans_get(
        &self,
        #[tool(param)]
        #[schemars(description = "Plan (budget) UUID")]
        plan_id: String,
    ) -> Result<String, String> {
        self.client
            .get_plan(&plan_id)
            .await
            .map(|d| to_json(&d))
            .map_err(|e| e.to_string())
    }

    #[tool(description = "Get plan settings (date and currency format)")]
    async fn ynab_plans_settings(
        &self,
        #[tool(param)]
        #[schemars(description = "Plan (budget) UUID")]
        plan_id: String,
    ) -> Result<String, String> {
        self.client
            .get_plan_settings(&plan_id)
            .await
            .map(|d| to_json(&d))
            .map_err(|e| e.to_string())
    }

    // === Accounts ===

    #[tool(
        description = "List all accounts in a YNAB plan. Returns names, types, balances in milliunits, and status. Supports delta sync."
    )]
    async fn ynab_accounts_list(
        &self,
        #[tool(param)]
        #[schemars(description = "Plan (budget) UUID")]
        plan_id: String,
        #[tool(param)]
        #[schemars(
            description = "Server knowledge number for delta sync. Pass server_knowledge from a previous response to get only changes."
        )]
        last_knowledge_of_server: Option<i64>,
    ) -> Result<String, String> {
        self.client
            .get_accounts(&plan_id, last_knowledge_of_server)
            .await
            .map(|d| to_json(&d))
            .map_err(|e| e.to_string())
    }

    #[tool(description = "Get a single account by ID")]
    async fn ynab_accounts_get(
        &self,
        #[tool(param)]
        #[schemars(description = "Plan (budget) UUID")]
        plan_id: String,
        #[tool(param)]
        #[schemars(description = "Account UUID")]
        account_id: String,
    ) -> Result<String, String> {
        self.client
            .get_account(&plan_id, &account_id)
            .await
            .map(|d| to_json(&d))
            .map_err(|e| e.to_string())
    }

    #[tool(
        description = "Create a new account. Types: checking, savings, cash, creditCard, lineOfCredit, otherAsset, otherLiability, mortgage, autoLoan, studentLoan, personalLoan, medicalDebt, otherDebt"
    )]
    async fn ynab_accounts_create(
        &self,
        #[tool(param)]
        #[schemars(description = "Plan (budget) UUID")]
        plan_id: String,
        #[tool(param)]
        #[schemars(description = "Account name")]
        name: String,
        #[tool(param)]
        #[schemars(
            description = "Account type: checking, savings, cash, creditCard, lineOfCredit, otherAsset, otherLiability, mortgage, autoLoan, studentLoan, personalLoan, medicalDebt, otherDebt"
        )]
        account_type: String,
        #[tool(param)]
        #[schemars(description = "Opening balance in milliunits (1000 = $1.00)")]
        balance: i64,
    ) -> Result<String, String> {
        let acct_type: ynab_types::AccountType =
            serde_json::from_value(serde_json::json!(account_type))
                .map_err(|e| format!("Invalid account type: {e}"))?;
        let account = ynab_types::SaveAccount {
            name,
            account_type: acct_type,
            balance,
        };
        self.client
            .create_account(&plan_id, &account)
            .await
            .map(|d| to_json(&d))
            .map_err(|e| e.to_string())
    }

    // === Transactions ===

    #[tool(
        description = "List transactions in a plan. Supports date filtering, type filtering (uncategorized/unapproved), and delta sync."
    )]
    async fn ynab_transactions_list(
        &self,
        #[tool(param)]
        #[schemars(description = "Plan (budget) UUID")]
        plan_id: String,
        #[tool(param)]
        #[schemars(description = "Only return transactions on or after this date (YYYY-MM-DD)")]
        since_date: Option<String>,
        #[tool(param)]
        #[schemars(description = "Filter: 'uncategorized' or 'unapproved'")]
        transaction_type: Option<String>,
        #[tool(param)]
        #[schemars(description = "Server knowledge for delta sync")]
        last_knowledge_of_server: Option<i64>,
    ) -> Result<String, String> {
        self.client
            .get_transactions(
                &plan_id,
                since_date.as_deref(),
                transaction_type.as_deref(),
                last_knowledge_of_server,
            )
            .await
            .map(|d| to_json(&d))
            .map_err(|e| e.to_string())
    }

    #[tool(description = "Get a single transaction by ID with full details including subtransactions")]
    async fn ynab_transactions_get(
        &self,
        #[tool(param)]
        #[schemars(description = "Plan (budget) UUID")]
        plan_id: String,
        #[tool(param)]
        #[schemars(description = "Transaction UUID")]
        transaction_id: String,
    ) -> Result<String, String> {
        self.client
            .get_transaction(&plan_id, &transaction_id)
            .await
            .map(|d| to_json(&d))
            .map_err(|e| e.to_string())
    }

    #[tool(
        description = "Create a transaction. Provide JSON with: account_id (required), date (required, YYYY-MM-DD), amount (required, milliunits, negative=outflow), payee_id, payee_name (max 100 chars), category_id, memo (max 200 chars), cleared (cleared|uncleared|reconciled), approved, flag_color (red|orange|yellow|green|blue|purple), import_id, subtransactions"
    )]
    async fn ynab_transactions_create(
        &self,
        #[tool(param)]
        #[schemars(description = "Plan (budget) UUID")]
        plan_id: String,
        #[tool(param)]
        #[schemars(description = "Transaction JSON object")]
        transaction_json: String,
    ) -> Result<String, String> {
        let txn: ynab_types::SaveTransaction =
            serde_json::from_str(&transaction_json).map_err(|e| format!("Invalid JSON: {e}"))?;
        self.client
            .create_transaction(&plan_id, &txn)
            .await
            .map(|d| to_json(&d))
            .map_err(|e| e.to_string())
    }

    #[tool(description = "Update a transaction. Provide JSON with fields to change (same fields as create, all optional).")]
    async fn ynab_transactions_update(
        &self,
        #[tool(param)]
        #[schemars(description = "Plan (budget) UUID")]
        plan_id: String,
        #[tool(param)]
        #[schemars(description = "Transaction UUID to update")]
        transaction_id: String,
        #[tool(param)]
        #[schemars(description = "Updated fields as JSON object")]
        transaction_json: String,
    ) -> Result<String, String> {
        let txn: serde_json::Value =
            serde_json::from_str(&transaction_json).map_err(|e| format!("Invalid JSON: {e}"))?;
        self.client
            .update_transaction(&plan_id, &transaction_id, &txn)
            .await
            .map(|d| to_json(&d))
            .map_err(|e| e.to_string())
    }

    #[tool(description = "Update multiple transactions at once. Provide a JSON array of transaction objects (each must include id).")]
    async fn ynab_transactions_update_bulk(
        &self,
        #[tool(param)]
        #[schemars(description = "Plan (budget) UUID")]
        plan_id: String,
        #[tool(param)]
        #[schemars(description = "JSON array of transaction objects to update")]
        transactions_json: String,
    ) -> Result<String, String> {
        let txns: Vec<serde_json::Value> =
            serde_json::from_str(&transactions_json).map_err(|e| format!("Invalid JSON: {e}"))?;
        self.client
            .update_transactions_bulk(&plan_id, &txns)
            .await
            .map(|d| to_json(&d))
            .map_err(|e| e.to_string())
    }

    #[tool(description = "Delete a transaction by ID")]
    async fn ynab_transactions_delete(
        &self,
        #[tool(param)]
        #[schemars(description = "Plan (budget) UUID")]
        plan_id: String,
        #[tool(param)]
        #[schemars(description = "Transaction UUID to delete")]
        transaction_id: String,
    ) -> Result<String, String> {
        self.client
            .delete_transaction(&plan_id, &transaction_id)
            .await
            .map(|d| to_json(&d))
            .map_err(|e| e.to_string())
    }

    #[tool(
        description = "Import transactions from linked accounts. Uses bank-style deduplication via import_id."
    )]
    async fn ynab_transactions_import(
        &self,
        #[tool(param)]
        #[schemars(description = "Plan (budget) UUID")]
        plan_id: String,
    ) -> Result<String, String> {
        self.client
            .import_transactions(&plan_id)
            .await
            .map(|d| to_json(&d))
            .map_err(|e| e.to_string())
    }

    #[tool(description = "List transactions for a specific account. Supports date filtering and delta sync.")]
    async fn ynab_transactions_by_account(
        &self,
        #[tool(param)]
        #[schemars(description = "Plan (budget) UUID")]
        plan_id: String,
        #[tool(param)]
        #[schemars(description = "Account UUID")]
        account_id: String,
        #[tool(param)]
        #[schemars(description = "Only return transactions on or after this date (YYYY-MM-DD)")]
        since_date: Option<String>,
        #[tool(param)]
        #[schemars(description = "Server knowledge for delta sync")]
        last_knowledge_of_server: Option<i64>,
    ) -> Result<String, String> {
        self.client
            .get_transactions_by_account(
                &plan_id,
                &account_id,
                since_date.as_deref(),
                last_knowledge_of_server,
            )
            .await
            .map(|d| to_json(&d))
            .map_err(|e| e.to_string())
    }

    #[tool(description = "List transactions for a specific category. Supports date filtering and delta sync.")]
    async fn ynab_transactions_by_category(
        &self,
        #[tool(param)]
        #[schemars(description = "Plan (budget) UUID")]
        plan_id: String,
        #[tool(param)]
        #[schemars(description = "Category UUID")]
        category_id: String,
        #[tool(param)]
        #[schemars(description = "Only return transactions on or after this date (YYYY-MM-DD)")]
        since_date: Option<String>,
        #[tool(param)]
        #[schemars(description = "Server knowledge for delta sync")]
        last_knowledge_of_server: Option<i64>,
    ) -> Result<String, String> {
        self.client
            .get_transactions_by_category(
                &plan_id,
                &category_id,
                since_date.as_deref(),
                last_knowledge_of_server,
            )
            .await
            .map(|d| to_json(&d))
            .map_err(|e| e.to_string())
    }

    #[tool(description = "List transactions for a specific payee. Supports date filtering and delta sync.")]
    async fn ynab_transactions_by_payee(
        &self,
        #[tool(param)]
        #[schemars(description = "Plan (budget) UUID")]
        plan_id: String,
        #[tool(param)]
        #[schemars(description = "Payee UUID")]
        payee_id: String,
        #[tool(param)]
        #[schemars(description = "Only return transactions on or after this date (YYYY-MM-DD)")]
        since_date: Option<String>,
        #[tool(param)]
        #[schemars(description = "Server knowledge for delta sync")]
        last_knowledge_of_server: Option<i64>,
    ) -> Result<String, String> {
        self.client
            .get_transactions_by_payee(
                &plan_id,
                &payee_id,
                since_date.as_deref(),
                last_knowledge_of_server,
            )
            .await
            .map(|d| to_json(&d))
            .map_err(|e| e.to_string())
    }

    #[tool(description = "List transactions for a specific month. Supports delta sync.")]
    async fn ynab_transactions_by_month(
        &self,
        #[tool(param)]
        #[schemars(description = "Plan (budget) UUID")]
        plan_id: String,
        #[tool(param)]
        #[schemars(description = "Month in YYYY-MM-DD format (e.g., 2026-03-01)")]
        month: String,
        #[tool(param)]
        #[schemars(description = "Server knowledge for delta sync")]
        last_knowledge_of_server: Option<i64>,
    ) -> Result<String, String> {
        self.client
            .get_transactions_by_month(&plan_id, &month, last_knowledge_of_server)
            .await
            .map(|d| to_json(&d))
            .map_err(|e| e.to_string())
    }

    // === Categories ===

    #[tool(
        description = "List all categories grouped by category group. Supports delta sync."
    )]
    async fn ynab_categories_list(
        &self,
        #[tool(param)]
        #[schemars(description = "Plan (budget) UUID")]
        plan_id: String,
        #[tool(param)]
        #[schemars(description = "Server knowledge for delta sync")]
        last_knowledge_of_server: Option<i64>,
    ) -> Result<String, String> {
        self.client
            .get_categories(&plan_id, last_knowledge_of_server)
            .await
            .map(|d| to_json(&d))
            .map_err(|e| e.to_string())
    }

    #[tool(description = "Get a single category by ID")]
    async fn ynab_categories_get(
        &self,
        #[tool(param)]
        #[schemars(description = "Plan (budget) UUID")]
        plan_id: String,
        #[tool(param)]
        #[schemars(description = "Category UUID")]
        category_id: String,
    ) -> Result<String, String> {
        self.client
            .get_category(&plan_id, &category_id)
            .await
            .map(|d| to_json(&d))
            .map_err(|e| e.to_string())
    }

    // === Payees ===

    #[tool(description = "List all payees in a plan. Supports delta sync.")]
    async fn ynab_payees_list(
        &self,
        #[tool(param)]
        #[schemars(description = "Plan (budget) UUID")]
        plan_id: String,
        #[tool(param)]
        #[schemars(description = "Server knowledge for delta sync")]
        last_knowledge_of_server: Option<i64>,
    ) -> Result<String, String> {
        self.client
            .get_payees(&plan_id, last_knowledge_of_server)
            .await
            .map(|d| to_json(&d))
            .map_err(|e| e.to_string())
    }

    #[tool(description = "Get a single payee by ID")]
    async fn ynab_payees_get(
        &self,
        #[tool(param)]
        #[schemars(description = "Plan (budget) UUID")]
        plan_id: String,
        #[tool(param)]
        #[schemars(description = "Payee UUID")]
        payee_id: String,
    ) -> Result<String, String> {
        self.client
            .get_payee(&plan_id, &payee_id)
            .await
            .map(|d| to_json(&d))
            .map_err(|e| e.to_string())
    }

    // === Payee Locations ===

    #[tool(description = "List all payee locations in a plan")]
    async fn ynab_payee_locations_list(
        &self,
        #[tool(param)]
        #[schemars(description = "Plan (budget) UUID")]
        plan_id: String,
    ) -> Result<String, String> {
        self.client
            .get_payee_locations(&plan_id)
            .await
            .map(|d| to_json(&d))
            .map_err(|e| e.to_string())
    }

    #[tool(description = "Get a single payee location by ID")]
    async fn ynab_payee_locations_get(
        &self,
        #[tool(param)]
        #[schemars(description = "Plan (budget) UUID")]
        plan_id: String,
        #[tool(param)]
        #[schemars(description = "Payee location UUID")]
        payee_location_id: String,
    ) -> Result<String, String> {
        self.client
            .get_payee_location(&plan_id, &payee_location_id)
            .await
            .map(|d| to_json(&d))
            .map_err(|e| e.to_string())
    }

    #[tool(description = "List payee locations for a specific payee")]
    async fn ynab_payee_locations_by_payee(
        &self,
        #[tool(param)]
        #[schemars(description = "Plan (budget) UUID")]
        plan_id: String,
        #[tool(param)]
        #[schemars(description = "Payee UUID")]
        payee_id: String,
    ) -> Result<String, String> {
        self.client
            .get_payee_locations_by_payee(&plan_id, &payee_id)
            .await
            .map(|d| to_json(&d))
            .map_err(|e| e.to_string())
    }

    // === Months ===

    #[tool(description = "List all budget months. Returns monthly summaries with income, budgeted, activity, and to_be_budgeted amounts. Supports delta sync.")]
    async fn ynab_months_list(
        &self,
        #[tool(param)]
        #[schemars(description = "Plan (budget) UUID")]
        plan_id: String,
        #[tool(param)]
        #[schemars(description = "Server knowledge for delta sync")]
        last_knowledge_of_server: Option<i64>,
    ) -> Result<String, String> {
        self.client
            .get_months(&plan_id, last_knowledge_of_server)
            .await
            .map(|d| to_json(&d))
            .map_err(|e| e.to_string())
    }

    #[tool(description = "Get a single budget month with category details")]
    async fn ynab_months_get(
        &self,
        #[tool(param)]
        #[schemars(description = "Plan (budget) UUID")]
        plan_id: String,
        #[tool(param)]
        #[schemars(description = "Month in YYYY-MM-DD format (e.g., 2026-03-01)")]
        month: String,
    ) -> Result<String, String> {
        self.client
            .get_month(&plan_id, &month)
            .await
            .map(|d| to_json(&d))
            .map_err(|e| e.to_string())
    }

    // === Scheduled Transactions ===

    #[tool(description = "List all scheduled/recurring transactions. Supports delta sync.")]
    async fn ynab_scheduled_list(
        &self,
        #[tool(param)]
        #[schemars(description = "Plan (budget) UUID")]
        plan_id: String,
        #[tool(param)]
        #[schemars(description = "Server knowledge for delta sync")]
        last_knowledge_of_server: Option<i64>,
    ) -> Result<String, String> {
        self.client
            .get_scheduled_transactions(&plan_id, last_knowledge_of_server)
            .await
            .map(|d| to_json(&d))
            .map_err(|e| e.to_string())
    }

    #[tool(description = "Get a single scheduled transaction by ID")]
    async fn ynab_scheduled_get(
        &self,
        #[tool(param)]
        #[schemars(description = "Plan (budget) UUID")]
        plan_id: String,
        #[tool(param)]
        #[schemars(description = "Scheduled transaction UUID")]
        scheduled_transaction_id: String,
    ) -> Result<String, String> {
        self.client
            .get_scheduled_transaction(&plan_id, &scheduled_transaction_id)
            .await
            .map(|d| to_json(&d))
            .map_err(|e| e.to_string())
    }

    // === Money Movements ===

    #[tool(description = "List all money movements in a plan. Supports delta sync.")]
    async fn ynab_money_movements_list(
        &self,
        #[tool(param)]
        #[schemars(description = "Plan (budget) UUID")]
        plan_id: String,
        #[tool(param)]
        #[schemars(description = "Server knowledge for delta sync")]
        last_knowledge_of_server: Option<i64>,
    ) -> Result<String, String> {
        self.client
            .get_money_movements(&plan_id, last_knowledge_of_server)
            .await
            .map(|d| to_json(&d))
            .map_err(|e| e.to_string())
    }

    #[tool(description = "List money movement groups in a plan. Supports delta sync.")]
    async fn ynab_money_movements_groups(
        &self,
        #[tool(param)]
        #[schemars(description = "Plan (budget) UUID")]
        plan_id: String,
        #[tool(param)]
        #[schemars(description = "Server knowledge for delta sync")]
        last_knowledge_of_server: Option<i64>,
    ) -> Result<String, String> {
        self.client
            .get_money_movement_groups(&plan_id, last_knowledge_of_server)
            .await
            .map(|d| to_json(&d))
            .map_err(|e| e.to_string())
    }
}

#[tool(tool_box)]
impl ServerHandler for YnabMcpServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            instructions: Some(INSTRUCTIONS.into()),
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            ..Default::default()
        }
    }
}
