# YNAB CLI

Command-line interface and MCP server for the YNAB (You Need A Budget) API. Designed for both humans and AI agents.

## Quick Start

```bash
# Authenticate with a personal access token
ynab auth login --pat

# List your budgets (plans)
ynab plans list

# List transactions
ynab transactions list --plan-id <PLAN_ID>

# Start MCP server for AI agent integration
ynab mcp
```

## Key Concepts

- **Plan ID**: Most commands require `--plan-id <uuid>` (or set `YNAB_PLAN_ID` env var)
- **Milliunits**: All monetary amounts are in milliunits. $25.00 = 25000. Negative = outflow.
- **Delta Sync**: Pass `--last-knowledge <N>` to get only changes since a previous response
- **Output Formats**: `--output-format json|table|csv` (default: json)
- **Dry Run**: `--dry-run` previews the HTTP request without executing

## Resources

| Resource | Commands |
|----------|----------|
| plans | list, get, settings |
| accounts | list, get, create |
| transactions | list, get, create, update, update-bulk, delete, import, by-account, by-category, by-payee, by-month |
| categories | list, get, month-get |
| payees | list, get |
| payee-locations | list, get, by-payee |
| months | list, get |
| scheduled | list, get |
| money-movements | list, by-month, groups, groups-by-month |

## MCP Server

Start with `ynab mcp`. Provides 31 tools (one per API endpoint) over stdio transport. Configure in Claude Desktop:

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

## Authentication

Token resolution order:
1. `YNAB_ACCESS_TOKEN` environment variable
2. `--token` flag
3. macOS Keychain (via `ynab auth login`)
4. `~/.config/ynab/credentials.json` fallback

## See Also

- [skills/transactions.md](skills/transactions.md) - Transaction workflows
- [skills/budgeting.md](skills/budgeting.md) - Budget management workflows
- [skills/reporting.md](skills/reporting.md) - Reporting and analysis workflows
