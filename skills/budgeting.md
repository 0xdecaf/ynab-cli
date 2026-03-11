# Budget Management Workflows

## View Budget Overview

```bash
# List all plans (budgets)
ynab plans list

# Get full budget details
ynab plans get --plan-id <PLAN_ID>

# Get budget settings (date/currency format)
ynab plans settings --plan-id <PLAN_ID>
```

## View Monthly Budget

```bash
# List all budget months
ynab months list --plan-id <PLAN_ID>

# Get a specific month with category details
ynab months get --plan-id <PLAN_ID> --month 2026-03-01
```

The month detail response includes per-category budgeted, activity, and balance amounts.

## View Categories

```bash
# List all categories grouped by category group
ynab categories list --plan-id <PLAN_ID>

# Get a single category
ynab categories get --plan-id <PLAN_ID> --category-id <CATEGORY_UUID>

# Get category details for a specific month
ynab categories month-get --plan-id <PLAN_ID> --month 2026-03-01 --category-id <CATEGORY_UUID>
```

## View Accounts

```bash
# List all accounts with balances
ynab accounts list --plan-id <PLAN_ID>

# Get a single account
ynab accounts get --plan-id <PLAN_ID> --account-id <ACCOUNT_UUID>

# Create a new account
ynab accounts create --plan-id <PLAN_ID> --json '{
  "name": "New Checking",
  "type": "checking",
  "balance": 100000
}'
```

Balance is in milliunits: 100000 = $100.00.

## View Scheduled Transactions

```bash
# List all recurring transactions
ynab scheduled list --plan-id <PLAN_ID>

# Get details of a scheduled transaction
ynab scheduled get --plan-id <PLAN_ID> --scheduled-transaction-id <UUID>
```

## View Payees

```bash
ynab payees list --plan-id <PLAN_ID>
ynab payees get --plan-id <PLAN_ID> --payee-id <PAYEE_UUID>
```

## Money Movements

```bash
# List budget moves/assignments
ynab money-movements list --plan-id <PLAN_ID>

# List money movement groups
ynab money-movements groups --plan-id <PLAN_ID>
```
