# Feature Parity Design

Date: 2026-03-10

## Goal

Close feature gaps identified by comparing against stephendolan/ynab-cli.

## Features

### 1. Default Plan ID
- `plans set-default <id>` writes to `~/.config/ynab/config.json`
- Resolution: `--plan-id` flag > `YNAB_PLAN_ID` env > config default
- New `config` module in ynab-client for reading/writing config

### 2. Category Update + Budget
- `categories update --category-id <id> --json '{...}'` — PATCH category
- `categories budget --month YYYY-MM-DD --category-id <id> --budgeted <milliunits>` — PATCH month category budget
- Client methods: `update_category()`, `update_category_month()`

### 3. Payee Update
- `payees update --payee-id <id> --json '{...}'` — PATCH payee
- Client method: `update_payee()`

### 4. Scheduled Transaction Delete
- `scheduled delete --scheduled-transaction-id <id>` — DELETE
- Client method: `delete_scheduled_transaction()`

### 5. Transaction Search
- `transactions search --memo <text> --payee-name <text> --since-date YYYY-MM-DD`
- Client-side: fetch all transactions, filter by memo/payee_name substring (case-insensitive)

### 6. Raw API Passthrough
- `api <METHOD> <PATH> [--body '{}']`
- Methods: GET, POST, PUT, PATCH, DELETE
- Returns raw JSON response body
- New `Api` command variant + `commands/api.rs`
- Client method: `raw_request(method, path, body)`

### 7. Dollar Conversion
- `--dollars` global flag
- Post-processes JSON output: fields ending in `amount`, `balance`, `budgeted`, `activity` get divided by 1000
- Table/CSV: formats as currency strings
- Applied in output module before rendering

### 8. Field Selection + Output to File
- `--fields id,date,amount` global flag — filters output to specified keys
- `--output <path>` global flag — writes to file instead of stdout
- Applied in output module

### MCP Updates
- Add tools: `ynab_categories_update`, `ynab_categories_budget`, `ynab_payees_update`, `ynab_scheduled_delete`, `ynab_transactions_search`, `ynab_api_raw`

### Skill File
- Create a comprehensive skill that covers installation, all commands, and common workflows
