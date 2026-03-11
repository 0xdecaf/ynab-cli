# Budget Management Workflows

## View Budget Overview

```bash
# List all plans (budgets)
ynab plans list

# Get full budget details
ynab plans get

# Get budget settings (date/currency format)
ynab plans settings
```

## Set Default Plan

```bash
# Set once so you don't need --plan-id every time
ynab plans list                     # Find your plan ID
ynab plans set-default <PLAN_ID>    # Save to ~/.config/ynab/config.json

# Now all commands use this plan automatically
ynab accounts list                  # No --plan-id needed
```

Plan ID resolution order: `--plan-id` flag > `YNAB_PLAN_ID` env > config default.

## View Monthly Budget

```bash
# List all budget months
ynab months list

# Get a specific month with category details
ynab months get --month 2026-03-01

# With dollar amounts
ynab months get --month 2026-03-01 --dollars
```

The month detail response includes per-category budgeted, activity, and balance amounts.

## View and Manage Categories

```bash
# List all categories grouped by category group
ynab categories list

# Get a single category
ynab categories get --category-id <CATEGORY_UUID>

# Get category details for a specific month
ynab categories month-get --month 2026-03-01 --category-id <CATEGORY_UUID>

# Update a category (name, note, goal settings)
ynab categories update --category-id <UUID> --json '{"name":"Groceries & Food"}'
ynab categories update --category-id <UUID> --json '{"note":"Weekly budget for groceries"}'
```

## Set Monthly Budget for a Category

```bash
# Budget $500.00 for a category in March 2026
ynab categories budget --month 2026-03-01 --category-id <UUID> --budgeted 500000

# Budget $0 to clear the assignment
ynab categories budget --month 2026-03-01 --category-id <UUID> --budgeted 0
```

The `--budgeted` value is in milliunits: 500000 = $500.00.

## View Accounts

```bash
# List all accounts with balances
ynab accounts list

# List with dollar amounts and specific fields
ynab accounts list --dollars --fields "name,balance,type"

# Get a single account
ynab accounts get --account-id <ACCOUNT_UUID>

# Create a new account
ynab accounts create --json '{
  "name": "New Checking",
  "type": "checking",
  "balance": 100000
}'
```

Balance is in milliunits: 100000 = $100.00.

## Manage Scheduled Transactions

```bash
# List all recurring transactions
ynab scheduled list

# Get details of a scheduled transaction
ynab scheduled get --scheduled-transaction-id <UUID>

# Delete a scheduled transaction
ynab scheduled delete --scheduled-transaction-id <UUID>
```

## Manage Payees

```bash
# List all payees
ynab payees list

# Rename a payee
ynab payees update --payee-id <UUID> --json '{"name":"Starbucks Coffee"}'
```

## Money Movements

```bash
# List budget moves/assignments
ynab money-movements list

# List money movement groups
ynab money-movements groups

# By specific month
ynab money-movements by-month --month 2026-03-01
ynab money-movements groups-by-month --month 2026-03-01
```
