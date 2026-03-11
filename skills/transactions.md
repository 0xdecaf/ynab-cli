# Transaction Workflows

## Create a Transaction

```bash
ynab transactions create --plan-id <PLAN_ID> --json '{
  "account_id": "<ACCOUNT_UUID>",
  "date": "2026-03-10",
  "amount": -25000,
  "payee_name": "Coffee Shop",
  "category_id": "<CATEGORY_UUID>",
  "memo": "Morning coffee",
  "cleared": "cleared"
}'
```

Amount is in milliunits: -25000 = -$25.00 (outflow). Positive = inflow.

## Split Transaction

```bash
ynab transactions create --plan-id <PLAN_ID> --json '{
  "account_id": "<ACCOUNT_UUID>",
  "date": "2026-03-10",
  "amount": -50000,
  "payee_name": "Grocery Store",
  "subtransactions": [
    { "amount": -30000, "category_id": "<GROCERIES_CAT>", "memo": "Food" },
    { "amount": -20000, "category_id": "<HOUSEHOLD_CAT>", "memo": "Cleaning supplies" }
  ]
}'
```

Subtransaction amounts must sum to the parent amount.

## Bulk Update Transactions

```bash
ynab transactions update-bulk --plan-id <PLAN_ID> --json '[
  { "id": "<TXN_1>", "category_id": "<CAT_ID>", "approved": true },
  { "id": "<TXN_2>", "category_id": "<CAT_ID>", "approved": true }
]'
```

## Find Uncategorized Transactions

```bash
ynab transactions list --plan-id <PLAN_ID> --type uncategorized
```

## Find Unapproved Transactions

```bash
ynab transactions list --plan-id <PLAN_ID> --type unapproved
```

## Transactions by Date Range

```bash
ynab transactions list --plan-id <PLAN_ID> --since-date 2026-03-01
```

## Transactions by Account/Category/Payee

```bash
ynab transactions by-account --plan-id <PLAN_ID> --account-id <ACCOUNT_UUID>
ynab transactions by-category --plan-id <PLAN_ID> --category-id <CATEGORY_UUID>
ynab transactions by-payee --plan-id <PLAN_ID> --payee-id <PAYEE_UUID>
```

## Delta Sync (Incremental Updates)

```bash
# First call - get all transactions
ynab transactions list --plan-id <PLAN_ID>
# Response includes "server_knowledge": 12345

# Subsequent calls - get only changes
ynab transactions list --plan-id <PLAN_ID> --last-knowledge 12345
```
