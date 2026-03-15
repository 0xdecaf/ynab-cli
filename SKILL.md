# YNAB CLI

Command-line interface and MCP server for the YNAB (You Need A Budget) API. Built in Rust. Designed for both humans and AI agents.

## Installation

### Download latest release (macOS/Linux)

```bash
# Detect platform and download latest release
REPO="0xdecaf/ynab-cli"
TAG=$(curl -sL "https://api.github.com/repos/$REPO/releases/latest" | grep '"tag_name"' | cut -d'"' -f4)
ARCH=$(uname -m)
OS=$(uname -s | tr '[:upper:]' '[:lower:]')
case "$ARCH" in
  x86_64)  TARGET="x86_64" ;;
  arm64|aarch64) TARGET="aarch64" ;;
esac
case "$OS" in
  darwin) TRIPLE="${TARGET}-apple-darwin" ;;
  linux)  TRIPLE="${TARGET}-unknown-linux-musl" ;;
esac
curl -sL "https://github.com/$REPO/releases/download/$TAG/ynab-cli-${TRIPLE}.tar.gz" | tar xz
sudo mv ynab /usr/local/bin/
```

### Build from source

```bash
git clone https://github.com/0xdecaf/ynab-cli.git
cd ynab-cli
cargo build --release
# Binary at target/release/ynab
```

## Authentication

```bash
# Store a Personal Access Token (recommended)
ynab auth login --pat
# Prompts for token, stores in macOS Keychain (or ~/.config/ynab/credentials.json)

# Check auth status
ynab auth status

# Remove stored credentials
ynab auth logout
```

Token resolution order:
1. `YNAB_ACCESS_TOKEN` environment variable
2. `--token <TOKEN>` flag
3. macOS Keychain (via `ynab auth login`)
4. `~/.config/ynab/credentials.json` fallback

## First-Time Setup

```bash
# 1. Authenticate
ynab auth login --pat

# 2. List plans and find your plan ID
ynab plans list

# 3. Set a default plan so you don't need --plan-id every time
ynab plans set-default <PLAN_ID>

# 4. Verify it works
ynab accounts list
```

## Key Concepts

- **Plan ID**: Most commands require a plan (budget) ID. Set once with `plans set-default`, pass via `--plan-id`, or set `YNAB_PLAN_ID` env var.
- **Milliunits**: All monetary amounts are in milliunits by default. $25.00 = 25000, -$10.50 = -10500 (negative = outflow). Use `--dollars` to auto-convert.
- **Delta Sync**: Pass `--last-knowledge <N>` to get only changes since a previous response's `server_knowledge` value.
- **Dry Run**: `--dry-run` previews the HTTP request without executing.
- **Schema**: `ynab schema <resource.method>` shows the request/response schema for any endpoint.

## Global Flags

| Flag | Description |
|------|-------------|
| `--plan-id <ID>` | Plan (budget) ID (env: `YNAB_PLAN_ID`, or set default) |
| `--output-format json\|table\|csv` | Output format (default: json) |
| `--dollars` | Convert milliunits to dollars in output |
| `--fields <FIELDS>` | Filter output to specified fields (comma-separated) |
| `--output <FILE>` | Write output to file instead of stdout |
| `--dry-run` | Preview HTTP request without executing |
| `--token <TOKEN>` | Override access token |
| `--verbose` | Show HTTP request/response details |

## Complete Command Reference

### auth — Authentication

```bash
ynab auth login --pat        # Store personal access token
ynab auth status             # Show current auth status
ynab auth logout             # Remove stored credentials
```

### user — User Info

```bash
ynab user get                # Get authenticated user info
```

### plans — Plans (Budgets)

```bash
ynab plans list              # List all plans
ynab plans get               # Get plan details (uses default or --plan-id)
ynab plans settings          # Get plan settings (date/currency format)
ynab plans set-default <ID>  # Save default plan ID to config
```

### accounts — Accounts

```bash
ynab accounts list                                    # List all accounts
ynab accounts get --account-id <UUID>                  # Get single account
ynab accounts create --json '{"name":"Checking","type":"checking","balance":100000}'
```

### transactions — Transactions

```bash
# Read
ynab transactions list                                 # List all transactions
ynab transactions list --since-date 2026-03-01         # By date
ynab transactions list --type uncategorized            # Uncategorized only
ynab transactions list --type unapproved               # Unapproved only
ynab transactions list --last-knowledge 12345          # Delta sync
ynab transactions get --transaction-id <UUID>          # Single transaction
ynab transactions by-account --account-id <UUID>       # By account
ynab transactions by-category --category-id <UUID>     # By category
ynab transactions by-payee --payee-id <UUID>           # By payee
ynab transactions by-month --month 2026-03-01          # By month

# Search (client-side filtering)
ynab transactions search --memo "coffee"               # Search by memo text
ynab transactions search --payee-name "Starbucks"      # Search by payee name
ynab transactions search --min-amount -50000           # Minimum amount filter
ynab transactions search --max-amount -1000            # Maximum amount filter
ynab transactions search --since-date 2026-01-01       # Combined with date

# Write
ynab transactions create --json '{
  "account_id": "<UUID>",
  "date": "2026-03-10",
  "amount": -25000,
  "payee_name": "Coffee Shop",
  "category_id": "<UUID>",
  "memo": "Morning coffee",
  "cleared": "cleared"
}'
ynab transactions update --transaction-id <UUID> --json '{"memo":"Updated"}'
ynab transactions update-bulk --json '[{"id":"<UUID>","approved":true}]'
ynab transactions delete --transaction-id <UUID>
ynab transactions import --json '[...]'                # Bank-style deduplication
```

### categories — Categories

```bash
ynab categories list                                   # All categories by group
ynab categories get --category-id <UUID>               # Single category
ynab categories month-get --month 2026-03-01 --category-id <UUID>  # Category for month
ynab categories update --category-id <UUID> --json '{"name":"New Name"}'
ynab categories budget --month 2026-03-01 --category-id <UUID> --budgeted 500000
# Sets budgeted amount for March 2026 to $500.00 (500000 milliunits)
```

### payees — Payees

```bash
ynab payees list                                       # List all payees
ynab payees get --payee-id <UUID>                      # Single payee
ynab payees update --payee-id <UUID> --json '{"name":"New Name"}'
```

### payee-locations — Payee Locations

```bash
ynab payee-locations list                              # All payee locations
ynab payee-locations get --payee-location-id <UUID>    # Single location
ynab payee-locations by-payee --payee-id <UUID>        # Locations for payee
```

### months — Monthly Summaries

```bash
ynab months list                                       # All months overview
ynab months get --month 2026-03-01                     # Month detail with categories
```

### scheduled — Scheduled Transactions

```bash
ynab scheduled list                                    # All scheduled transactions
ynab scheduled get --scheduled-transaction-id <UUID>   # Single scheduled txn
ynab scheduled delete --scheduled-transaction-id <UUID> # Delete scheduled txn
```

### money-movements — Money Movements

```bash
ynab money-movements list                              # All money movements
ynab money-movements by-month --month 2026-03-01       # By month
ynab money-movements groups                            # Movement groups
ynab money-movements groups-by-month --month 2026-03-01
```

### schema — Schema Introspection

```bash
ynab schema transactions.create   # View request schema for creating transactions
ynab schema plans.list            # View schema for listing plans
ynab schema accounts.get          # View schema for any resource.method
```

### api — Raw API Passthrough

```bash
# Access any YNAB API endpoint directly
ynab api GET /v1/budgets
ynab api GET /v1/budgets/<PLAN_ID>/accounts
ynab api POST /v1/budgets/<PLAN_ID>/transactions --body '{"transaction":{...}}'
ynab api PATCH /v1/budgets/<PLAN_ID>/categories/<ID> --body '{"category":{"name":"New"}}'
ynab api DELETE /v1/budgets/<PLAN_ID>/scheduled_transactions/<ID>
```

### completions — Shell Completions

```bash
ynab completions bash > ~/.bash_completion.d/ynab
ynab completions zsh > ~/.zfunc/_ynab
ynab completions fish > ~/.config/fish/completions/ynab.fish
```

## Output Features

### Dollar conversion

```bash
# Default: milliunits
ynab accounts list
# {"balance": 1500000, ...}

# With --dollars: human-readable
ynab accounts list --dollars
# {"balance": 1500.0, ...}
```

### Field filtering

```bash
# Show only specific fields
ynab transactions list --fields "id,date,amount,payee_name"
ynab accounts list --fields "name,balance" --dollars
```

### File output

```bash
# Write to file
ynab transactions list --output transactions.json
ynab transactions list --output-format csv --output spending.csv
```

### Table and CSV output

```bash
ynab accounts list --output-format table   # Human-readable table
ynab accounts list --output-format csv     # For spreadsheets
```

## MCP Server

Start with `ynab mcp`. Provides 37 tools over stdio transport.

### Configuration for Claude Desktop / Claude Code

```json
{
  "mcpServers": {
    "ynab": {
      "command": "ynab",
      "args": ["mcp"],
      "env": {
        "YNAB_ACCESS_TOKEN": "your-token-here"
      }
    }
  }
}
```

### Available MCP Tools (37 total)

| Tool | Description |
|------|-------------|
| `ynab_user_get` | Get authenticated user info |
| `ynab_plans_list` | List all plans |
| `ynab_plans_get` | Get plan details |
| `ynab_plans_settings` | Get plan settings |
| `ynab_accounts_list` | List accounts |
| `ynab_accounts_get` | Get single account |
| `ynab_accounts_create` | Create account |
| `ynab_categories_list` | List categories |
| `ynab_categories_get` | Get single category |
| `ynab_categories_month_get` | Get category for month |
| `ynab_categories_update` | Update category |
| `ynab_categories_budget` | Set month budget for category |
| `ynab_transactions_list` | List transactions |
| `ynab_transactions_get` | Get single transaction |
| `ynab_transactions_create` | Create transaction |
| `ynab_transactions_update` | Update transaction |
| `ynab_transactions_update_bulk` | Bulk update transactions |
| `ynab_transactions_delete` | Delete transaction |
| `ynab_transactions_import` | Import transactions |
| `ynab_transactions_by_account` | Transactions by account |
| `ynab_transactions_by_category` | Transactions by category |
| `ynab_transactions_by_payee` | Transactions by payee |
| `ynab_transactions_by_month` | Transactions by month |
| `ynab_transactions_search` | Search by memo/payee/amount |
| `ynab_payees_list` | List payees |
| `ynab_payees_get` | Get single payee |
| `ynab_payees_update` | Update payee |
| `ynab_months_list` | List month summaries |
| `ynab_months_get` | Get month detail |
| `ynab_scheduled_list` | List scheduled transactions |
| `ynab_scheduled_get` | Get scheduled transaction |
| `ynab_scheduled_delete` | Delete scheduled transaction |
| `ynab_payee_locations_list` | List payee locations |
| `ynab_payee_locations_get` | Get payee location |
| `ynab_payee_locations_by_payee` | Locations by payee |
| `ynab_money_movements_list` | List money movements |
| `ynab_api_raw` | Raw API passthrough |

## See Also

- [skills/transactions.md](skills/transactions.md) — Transaction workflows
- [skills/budgeting.md](skills/budgeting.md) — Budget management workflows
- [skills/reporting.md](skills/reporting.md) — Reporting and analysis workflows
