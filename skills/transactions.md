# Transaction Workflows

## Create a Transaction

```bash
ynab transactions create --json '{
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
ynab transactions create --json '{
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
ynab transactions update-bulk --json '[
  { "id": "<TXN_1>", "category_id": "<CAT_ID>", "approved": true },
  { "id": "<TXN_2>", "category_id": "<CAT_ID>", "approved": true }
]'
```

## Search Transactions

Client-side filtering across all transactions. Filters can be combined.

```bash
# Search by memo text (case-insensitive)
ynab transactions search --memo "coffee"

# Search by payee name (case-insensitive)
ynab transactions search --payee-name "Starbucks"

# Filter by amount range (in milliunits)
ynab transactions search --min-amount -100000 --max-amount -5000

# Combine with date filter
ynab transactions search --memo "grocery" --since-date 2026-01-01

# Use --dollars for readable output
ynab transactions search --payee-name "Amazon" --dollars --fields "date,payee_name,amount"
```

## Find Uncategorized Transactions

```bash
ynab transactions list --type uncategorized
```

## Find Unapproved Transactions

```bash
ynab transactions list --type unapproved
```

## Transactions by Date Range

```bash
ynab transactions list --since-date 2026-03-01
```

## Transactions by Account/Category/Payee

```bash
ynab transactions by-account --account-id <ACCOUNT_UUID>
ynab transactions by-category --category-id <CATEGORY_UUID>
ynab transactions by-payee --payee-id <PAYEE_UUID>
ynab transactions by-month --month 2026-03-01
```

## Delete a Transaction

```bash
ynab transactions delete --transaction-id <UUID>
```

## Import Transactions (Bank-Style)

```bash
ynab transactions import --json '[{
  "account_id": "<UUID>",
  "date": "2026-03-10",
  "amount": -15000,
  "payee_name": "Gas Station",
  "import_id": "YNAB:15000:2026-03-10:1"
}]'
```

The `import_id` enables bank-style deduplication — re-importing the same `import_id` is a no-op.

## Delta Sync (Incremental Updates)

```bash
# First call - get all transactions
ynab transactions list
# Response includes "server_knowledge": 12345

# Subsequent calls - get only changes
ynab transactions list --last-knowledge 12345
```

## Export Transactions to CSV

```bash
ynab transactions list --output-format csv --output transactions.csv
ynab transactions list --output-format csv --dollars --fields "date,payee_name,amount,category_name" --output spending.csv
```
