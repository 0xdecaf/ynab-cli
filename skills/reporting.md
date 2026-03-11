# Reporting and Analysis Workflows

## Spending by Category

```bash
# Get transactions for a specific category
ynab transactions by-category --plan-id <PLAN_ID> --category-id <CATEGORY_UUID> --since-date 2026-03-01

# Get category budget vs actual for a month
ynab categories month-get --plan-id <PLAN_ID> --month 2026-03-01 --category-id <CATEGORY_UUID>
```

The category response includes `budgeted`, `activity`, and `balance` (all in milliunits).

## Spending by Payee

```bash
ynab transactions by-payee --plan-id <PLAN_ID> --payee-id <PAYEE_UUID> --since-date 2026-01-01
```

## Account Activity

```bash
ynab transactions by-account --plan-id <PLAN_ID> --account-id <ACCOUNT_UUID> --since-date 2026-03-01
```

## Monthly Summary

```bash
# Get all months at a glance
ynab months list --plan-id <PLAN_ID>

# Each month includes: income, budgeted, activity, to_be_budgeted, age_of_money
```

## Monthly Detail with Categories

```bash
ynab months get --plan-id <PLAN_ID> --month 2026-03-01
```

Returns every category for that month with budgeted/activity/balance.

## Output Formats

```bash
# JSON (default, best for agents)
ynab transactions list --plan-id <PLAN_ID> --output-format json

# Table (human-readable)
ynab transactions list --plan-id <PLAN_ID> --output-format table

# CSV (for spreadsheets/data analysis)
ynab transactions list --plan-id <PLAN_ID> --output-format csv
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
ynab transactions list --plan-id <PLAN_ID>
# Note the server_knowledge value from the response

# Step 2: Incremental updates
ynab transactions list --plan-id <PLAN_ID> --last-knowledge <SERVER_KNOWLEDGE>
# Returns only transactions changed since last fetch
```

Resources supporting delta sync: accounts, transactions, categories, payees, months, scheduled, money-movements.
