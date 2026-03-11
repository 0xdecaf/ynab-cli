<p align="center">
  <h1 align="center">ynab-cli</h1>
  <p align="center">
    The only YNAB CLI with a built-in MCP server.<br>
    Manage your budget from the command line — or let AI agents do it for you.
  </p>
</p>

<p align="center">
  <a href="https://github.com/0xdecaf/ynab-cli/actions/workflows/ci.yml"><img src="https://github.com/0xdecaf/ynab-cli/actions/workflows/ci.yml/badge.svg" alt="CI"></a>
  <a href="https://codecov.io/gh/0xdecaf/ynab-cli"><img src="https://codecov.io/gh/0xdecaf/ynab-cli/graph/badge.svg" alt="codecov"></a>
  <a href="https://github.com/0xdecaf/ynab-cli/releases/latest"><img src="https://img.shields.io/github/v/release/0xdecaf/ynab-cli" alt="GitHub Release"></a>
  <a href="https://www.npmjs.com/package/ynab-cli-rs"><img src="https://img.shields.io/npm/v/ynab-cli-rs" alt="npm"></a>
  <a href="LICENSE"><img src="https://img.shields.io/badge/license-MIT-blue.svg" alt="License: MIT"></a>
</p>

---

## Features

🤖 **AI-Native MCP Server** — 37 tools over stdio transport. Works with Claude Desktop, Claude Code, and any MCP-compatible agent. The only YNAB CLI that speaks MCP.

📊 **Flexible Output** — JSON (default), human-readable tables, or CSV. Pipe to `jq`, import into spreadsheets, or feed directly to agents.

💵 **Dollar Mode** — `--dollars` converts YNAB's milliunits to real currency. No more dividing by 1000 in your head.

🔍 **Transaction Search** — Find transactions by memo, payee name, or amount range with client-side filtering across your entire history.

📁 **Field Filtering & Export** — `--fields "date,payee_name,amount"` keeps output focused. `--output report.csv` writes directly to file.

⚡ **Single Binary** — Built in Rust. No runtime, no dependencies, no garbage collector. Starts instantly.

🔐 **Secure Auth** — Tokens stored in macOS Keychain (with JSON fallback). npm package ships with OIDC provenance attestation.

🔄 **Delta Sync** — Fetch only what changed since your last request. Efficient polling for automation workflows.

🧩 **Full API Coverage** — Every YNAB API endpoint has a dedicated command. Anything missing? `ynab api GET /v1/...` hits the API directly.

## Install

### npm (easiest)

```bash
npm install -g ynab-cli-rs
```

### GitHub Release

```bash
# Detect platform and install latest release
curl -sL https://api.github.com/repos/0xdecaf/ynab-cli/releases/latest \
  | grep "browser_download_url.*$(uname -m | sed 's/arm64/aarch64/').*$(uname -s | tr '[:upper:]' '[:lower:]')" \
  | cut -d'"' -f4 | xargs curl -sL | tar xz
sudo mv ynab /usr/local/bin/
```

### Build from source

```bash
git clone https://github.com/0xdecaf/ynab-cli.git && cd ynab-cli
cargo build --release
# Binary at target/release/ynab
```

## Quick Start

```bash
# 1. Authenticate with your YNAB personal access token
ynab auth login --pat

# 2. See your budgets
ynab plans list

# 3. Set a default so you never need --plan-id again
ynab plans set-default <PLAN_ID>

# 4. You're in
ynab accounts list --dollars
ynab transactions list --since-date 2026-01-01 --dollars
```

> Get a personal access token at [app.ynab.com/settings/developer](https://app.ynab.com/settings/developer)

## Usage

Every resource has its own subcommand. Here are the highlights — run `ynab <command> --help` for full details.

### Transactions

```bash
ynab transactions list                                    # all transactions
ynab transactions list --type uncategorized               # find uncategorized
ynab transactions search --payee-name "Starbucks"         # search by payee
ynab transactions search --memo "coffee" --dollars        # search by memo
ynab transactions create --json '{"account_id":"...","date":"2026-03-10","amount":-25000,"payee_name":"Coffee Shop"}'
ynab transactions update --transaction-id <ID> --json '{"memo":"Updated"}'
ynab transactions update-bulk --json '[{"id":"...","approved":true}]'
```

### Budget & Categories

```bash
ynab categories list                                      # all categories
ynab categories budget --month 2026-03-01 --category-id <ID> --budgeted 500000
ynab categories update --category-id <ID> --json '{"name":"Dining Out"}'
ynab months get --month 2026-03-01 --dollars              # monthly breakdown
```

### Accounts & Payees

```bash
ynab accounts list --dollars --fields "name,type,balance" # quick balance check
ynab accounts create --json '{"name":"Savings","type":"savings","balance":0}'
ynab payees update --payee-id <ID> --json '{"name":"New Name"}'
```

### Scheduled Transactions

```bash
ynab scheduled list                                       # all recurring
ynab scheduled delete --scheduled-transaction-id <ID>     # remove one
```

### Raw API Access

```bash
ynab api GET /v1/budgets
ynab api PATCH /v1/budgets/<ID>/categories/<ID> --body '{"category":{"name":"X"}}'
```

### Output Options

```bash
ynab transactions list --output-format table              # human-readable
ynab transactions list --output-format csv --output spending.csv  # export
ynab transactions list --dollars --fields "date,payee_name,amount"  # focused
ynab transactions list --dry-run                          # preview HTTP request
```

## MCP Server

Start the server:

```bash
ynab mcp
```

37 tools — one per API endpoint — over stdio transport. Every tool accepts `plan_id` as the first parameter and returns JSON.

### Claude Desktop

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

### Claude Code

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

### What agents can do

- "How much did I spend on dining this month?"
- "Approve all uncategorized transactions as groceries"
- "Budget $200 for entertainment next month"
- "Export this month's spending to CSV"
- "What's my account balance across all accounts?"

The MCP server gives agents full read/write access to your YNAB data through structured tool calls.

## Global Flags

| Flag | Description |
|------|-------------|
| `--plan-id <ID>` | Plan (budget) ID — or set `YNAB_PLAN_ID` env, or `plans set-default` |
| `--output-format` | `json` (default), `table`, or `csv` |
| `--dollars` | Convert milliunits to dollars |
| `--fields <F>` | Comma-separated field filter |
| `--output <FILE>` | Write to file instead of stdout |
| `--dry-run` | Preview HTTP request without executing |
| `--verbose` | Show HTTP request/response details |

## Authentication

Token resolution order:

1. `YNAB_ACCESS_TOKEN` environment variable
2. `--token` flag
3. macOS Keychain (via `ynab auth login`)
4. `~/.config/ynab/credentials.json` fallback

## Contributing

```bash
git clone https://github.com/0xdecaf/ynab-cli.git
cd ynab-cli
cargo build
cargo test --all
cargo clippy --all-targets --all-features
```

PRs welcome. Please run `cargo fmt --all` before submitting.

## License

[MIT](LICENSE)
