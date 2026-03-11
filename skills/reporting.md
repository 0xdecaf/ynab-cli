# Reporting and Analysis Workflows

## Spending by Category

```bash
# Get transactions for a specific category
ynab transactions by-category --category-id <CATEGORY_UUID> --since-date 2026-03-01

# Get category budget vs actual for a month
ynab categories month-get --month 2026-03-01 --category-id <CATEGORY_UUID>

# With dollar amounts and specific fields
ynab categories month-get --month 2026-03-01 --category-id <UUID> --dollars --fields "name,budgeted,activity,balance"
```

The category response includes `budgeted`, `activity`, and `balance` (all in milliunits, use `--dollars` to convert).

## Spending by Payee

```bash
ynab transactions by-payee --payee-id <PAYEE_UUID> --since-date 2026-01-01
```

## Search Spending

```bash
# Find all coffee purchases
ynab transactions search --memo "coffee" --dollars --fields "date,payee_name,amount"

# Find large purchases over $100
ynab transactions search --max-amount -100000 --dollars

# Find spending at a specific store this year
ynab transactions search --payee-name "Amazon" --since-date 2026-01-01 --dollars
```

## Account Activity

```bash
ynab transactions by-account --account-id <ACCOUNT_UUID> --since-date 2026-03-01
```

## Monthly Summary

```bash
# Get all months at a glance
ynab months list

# Each month includes: income, budgeted, activity, to_be_budgeted, age_of_money
ynab months list --dollars --fields "month,income,budgeted,activity,to_be_budgeted"
```

## Monthly Detail with Categories

```bash
ynab months get --month 2026-03-01
```

Returns every category for that month with budgeted/activity/balance.

## Export to CSV for Spreadsheets

```bash
# All transactions as CSV
ynab transactions list --output-format csv --output all_transactions.csv

# Spending report with dollar amounts
ynab transactions list --since-date 2026-01-01 --output-format csv --dollars --fields "date,payee_name,category_name,amount,memo" --output spending_report.csv

# Account balances
ynab accounts list --output-format csv --dollars --fields "name,type,balance" --output accounts.csv

# Category budgets for a month
ynab months get --month 2026-03-01 --output-format csv --dollars --output march_budget.csv
```

## Output Formats

```bash
# JSON (default, best for agents)
ynab transactions list --output-format json

# Table (human-readable)
ynab transactions list --output-format table

# CSV (for spreadsheets/data analysis)
ynab transactions list --output-format csv
```

## Schema Introspection

```bash
# View the schema for any resource.method
ynab schema transactions.create
ynab schema plans.list
ynab schema accounts.list
```

## Efficient Polling with Delta Sync

For monitoring changes without re-fetching everything:

```bash
# Step 1: Full fetch
ynab transactions list
# Note the server_knowledge value from the response

# Step 2: Incremental updates
ynab transactions list --last-knowledge <SERVER_KNOWLEDGE>
# Returns only transactions changed since last fetch
```

Resources supporting delta sync: accounts, transactions, categories, payees, months, scheduled, money-movements.

## Raw API Access

For any endpoint not covered by specific commands:

```bash
ynab api GET /v1/budgets
ynab api GET /v1/budgets/<PLAN_ID>/accounts
ynab api POST /v1/budgets/<PLAN_ID>/transactions --body '{"transaction":{...}}'
```
