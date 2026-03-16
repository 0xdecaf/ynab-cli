use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(
    name = "ynab",
    about = "CLI and MCP server for the YNAB (You Need A Budget) API",
    long_about = "Command-line interface for the YNAB API. Designed for both humans and AI agents.\n\n\
        All monetary amounts are in milliunits (divide by 1000 for currency units).\n\
        For example, $25.00 is represented as 25000.\n\n\
        Quick start:\n  \
        ynab auth login --pat\n  \
        ynab plans list\n  \
        ynab transactions list --plan-id <id>",
    version
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,

    /// Plan (budget) ID. Required for most commands.
    #[arg(long, global = true, env = "YNAB_PLAN_ID")]
    pub plan_id: Option<String>,

    /// Output format
    #[arg(long, global = true, default_value = "json")]
    pub output_format: OutputFormat,

    /// Preview the HTTP request without executing
    #[arg(long, global = true)]
    pub dry_run: bool,

    /// Override access token
    #[arg(long, global = true, env = "YNAB_ACCESS_TOKEN", hide_env_values = true)]
    pub token: Option<String>,

    /// Show HTTP request/response details
    #[arg(long, global = true)]
    pub verbose: bool,

    /// Convert milliunit amounts to dollars (divide by 1000)
    #[arg(long, global = true)]
    pub dollars: bool,

    /// Filter output to specified fields (comma-separated, e.g., "id,date,amount")
    #[arg(long, global = true)]
    pub fields: Option<String>,

    /// Write output to a file instead of stdout
    #[arg(long, global = true)]
    pub output: Option<String>,
}

#[derive(Clone, ValueEnum)]
pub enum OutputFormat {
    Json,
    Table,
    Csv,
}

#[derive(Subcommand)]
pub enum Command {
    /// Manage authentication (login, logout, status)
    Auth {
        #[command(subcommand)]
        command: AuthCommand,
    },

    /// Get authenticated user information
    User {
        #[command(subcommand)]
        command: UserCommand,
    },

    /// List and view plans (budgets)
    Plans {
        #[command(subcommand)]
        command: PlansCommand,
    },

    /// Manage budget accounts
    Accounts {
        #[command(subcommand)]
        command: AccountsCommand,
    },

    /// Create, list, update, and delete transactions
    Transactions {
        #[command(subcommand)]
        command: TransactionsCommand,
    },

    /// Manage budget categories and category groups
    Categories {
        #[command(subcommand)]
        command: CategoriesCommand,
    },

    /// Manage payees
    Payees {
        #[command(subcommand)]
        command: PayeesCommand,
    },

    /// View payee location data
    PayeeLocations {
        #[command(subcommand)]
        command: PayeeLocationsCommand,
    },

    /// View monthly budget summaries
    Months {
        #[command(subcommand)]
        command: MonthsCommand,
    },

    /// Manage scheduled/recurring transactions
    Scheduled {
        #[command(subcommand)]
        command: ScheduledCommand,
    },

    /// View money movement data
    MoneyMovements {
        #[command(subcommand)]
        command: MoneyMovementsCommand,
    },

    /// Inspect API request/response schemas
    Schema {
        /// Resource and method (e.g., "transactions.create", "plans.list")
        resource_method: String,
    },

    /// Start MCP server for AI agent integration (stdio transport)
    Mcp,

    /// Generate shell completions
    Completions {
        /// Shell to generate completions for
        #[arg(value_enum)]
        shell: clap_complete::Shell,
    },

    /// Make a raw API request (any method, any path)
    Api {
        /// HTTP method (GET, POST, PUT, PATCH, DELETE)
        method: String,
        /// API path (e.g., /v1/plans)
        path: String,
        /// Request body as JSON
        #[arg(long)]
        body: Option<String>,
    },
}

// --- Auth ---

#[derive(Subcommand)]
pub enum AuthCommand {
    /// Log in via OAuth (opens browser) or with a personal access token (--pat)
    Login {
        /// Use a personal access token instead of OAuth
        #[arg(long)]
        pat: bool,

        /// Token value (will prompt if not provided, only used with --pat)
        #[arg(long)]
        token: Option<String>,
    },
    /// Clear stored credentials
    Logout,
    /// Show current authentication status
    Status,
    /// Print the current access token to stdout
    Token,
}

// --- User ---

#[derive(Subcommand)]
pub enum UserCommand {
    /// Get the authenticated user
    Get,
}

// --- Plans ---

#[derive(Subcommand)]
pub enum PlansCommand {
    /// List all plans (budgets)
    List,
    /// Get a single plan by ID
    Get {
        /// Plan ID (uses --plan-id if not specified)
        #[arg(long)]
        id: Option<String>,
    },
    /// Get plan settings
    Settings {
        /// Plan ID (uses --plan-id if not specified)
        #[arg(long)]
        id: Option<String>,
    },
    /// Set the default plan ID (saved to config)
    SetDefault {
        /// Plan ID to set as default
        id: String,
    },
}

// --- Accounts ---

#[derive(Subcommand)]
pub enum AccountsCommand {
    /// List all accounts in a plan
    List {
        /// Delta sync: server knowledge number from a previous response
        #[arg(long)]
        last_knowledge: Option<i64>,
    },
    /// Get a single account by ID
    Get {
        /// Account ID
        #[arg(long)]
        account_id: String,
    },
    /// Create a new account
    Create {
        /// Account data as JSON: {"name": "...", "type": "checking", "balance": 0}
        #[arg(long)]
        json: String,
    },
}

// --- Transactions ---

#[derive(Subcommand)]
pub enum TransactionsCommand {
    /// List transactions (supports delta sync)
    List {
        /// Only return transactions on or after this date (YYYY-MM-DD)
        #[arg(long)]
        since_date: Option<String>,
        /// Filter type: "uncategorized" or "unapproved"
        #[arg(long, rename_all = "lowercase")]
        r#type: Option<String>,
        /// Delta sync: server knowledge number
        #[arg(long)]
        last_knowledge: Option<i64>,
    },
    /// Get a single transaction by ID
    Get {
        /// Transaction ID
        #[arg(long)]
        transaction_id: String,
    },
    /// Create a transaction
    Create {
        /// Transaction data as JSON
        #[arg(long)]
        json: String,
    },
    /// Update a transaction by ID
    Update {
        /// Transaction ID
        #[arg(long)]
        transaction_id: String,
        /// Updated transaction data as JSON
        #[arg(long)]
        json: String,
    },
    /// Update multiple transactions at once
    UpdateBulk {
        /// Array of transactions as JSON
        #[arg(long)]
        json: String,
    },
    /// Delete a transaction by ID
    Delete {
        /// Transaction ID
        #[arg(long)]
        transaction_id: String,
    },
    /// Import transactions (bank-style deduplication)
    Import,
    /// List transactions for a specific account
    ByAccount {
        /// Account ID
        #[arg(long)]
        account_id: String,
        #[arg(long)]
        since_date: Option<String>,
        #[arg(long)]
        last_knowledge: Option<i64>,
    },
    /// List transactions for a specific category
    ByCategory {
        /// Category ID
        #[arg(long)]
        category_id: String,
        #[arg(long)]
        since_date: Option<String>,
        #[arg(long)]
        last_knowledge: Option<i64>,
    },
    /// List transactions for a specific payee
    ByPayee {
        /// Payee ID
        #[arg(long)]
        payee_id: String,
        #[arg(long)]
        since_date: Option<String>,
        #[arg(long)]
        last_knowledge: Option<i64>,
    },
    /// List transactions for a specific month
    ByMonth {
        /// Month in YYYY-MM-DD format (e.g., 2026-03-01)
        #[arg(long)]
        month: String,
        #[arg(long)]
        last_knowledge: Option<i64>,
    },
    /// Search transactions by memo or payee name (client-side filtering)
    Search {
        /// Search in memo field (case-insensitive substring match)
        #[arg(long)]
        memo: Option<String>,
        /// Search in payee name field (case-insensitive substring match)
        #[arg(long)]
        payee_name: Option<String>,
        /// Only return transactions on or after this date (YYYY-MM-DD)
        #[arg(long)]
        since_date: Option<String>,
        /// Maximum amount in milliunits
        #[arg(long)]
        max_amount: Option<i64>,
        /// Minimum amount in milliunits
        #[arg(long)]
        min_amount: Option<i64>,
    },
}

// --- Categories ---

#[derive(Subcommand)]
pub enum CategoriesCommand {
    /// List all categories grouped by category group
    List {
        #[arg(long)]
        last_knowledge: Option<i64>,
    },
    /// Get a single category by ID
    Get {
        #[arg(long)]
        category_id: String,
    },
    /// Get a category for a specific month
    MonthGet {
        #[arg(long)]
        month: String,
        #[arg(long)]
        category_id: String,
    },
    /// Update a category (name, note, etc.)
    Update {
        /// Category ID to update
        #[arg(long)]
        category_id: String,
        /// Updated category data as JSON
        #[arg(long)]
        json: String,
    },
    /// Set the budgeted amount for a category in a specific month
    Budget {
        /// Month in YYYY-MM-DD format (e.g., 2026-03-01)
        #[arg(long)]
        month: String,
        /// Category ID
        #[arg(long)]
        category_id: String,
        /// Budgeted amount in milliunits (1000 = $1.00)
        #[arg(long)]
        budgeted: i64,
    },
}

// --- Payees ---

#[derive(Subcommand)]
pub enum PayeesCommand {
    /// List all payees
    List {
        #[arg(long)]
        last_knowledge: Option<i64>,
    },
    /// Get a single payee by ID
    Get {
        #[arg(long)]
        payee_id: String,
    },
    /// Update a payee (rename)
    Update {
        /// Payee ID to update
        #[arg(long)]
        payee_id: String,
        /// Updated payee data as JSON (e.g., '{"name": "New Name"}')
        #[arg(long)]
        json: String,
    },
}

// --- Payee Locations ---

#[derive(Subcommand)]
pub enum PayeeLocationsCommand {
    /// List all payee locations
    List,
    /// Get a payee location by ID
    Get {
        #[arg(long)]
        payee_location_id: String,
    },
    /// List payee locations for a specific payee
    ByPayee {
        #[arg(long)]
        payee_id: String,
    },
}

// --- Months ---

#[derive(Subcommand)]
pub enum MonthsCommand {
    /// List all budget months
    List {
        #[arg(long)]
        last_knowledge: Option<i64>,
    },
    /// Get a single budget month
    Get {
        /// Month in YYYY-MM-DD format (e.g., 2026-03-01)
        #[arg(long)]
        month: String,
    },
}

// --- Scheduled Transactions ---

#[derive(Subcommand)]
pub enum ScheduledCommand {
    /// List all scheduled transactions
    List {
        #[arg(long)]
        last_knowledge: Option<i64>,
    },
    /// Get a single scheduled transaction
    Get {
        #[arg(long)]
        scheduled_transaction_id: String,
    },
    /// Delete a scheduled transaction
    Delete {
        #[arg(long)]
        scheduled_transaction_id: String,
    },
}

// --- Money Movements ---

#[derive(Subcommand)]
pub enum MoneyMovementsCommand {
    /// List all money movements
    List {
        #[arg(long)]
        last_knowledge: Option<i64>,
    },
    /// List money movements for a specific month
    ByMonth {
        #[arg(long)]
        month: String,
        #[arg(long)]
        last_knowledge: Option<i64>,
    },
    /// List money movement groups
    Groups {
        #[arg(long)]
        last_knowledge: Option<i64>,
    },
    /// List money movement groups for a specific month
    GroupsByMonth {
        #[arg(long)]
        month: String,
        #[arg(long)]
        last_knowledge: Option<i64>,
    },
}
