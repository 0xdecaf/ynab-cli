# ynab-cli-rs

Command-line interface and MCP server for the [YNAB](https://www.ynab.com) (You Need A Budget) API. Built in Rust, distributed as a single binary.

## Install

```bash
npm install -g ynab-cli-rs
```

Or run directly:

```bash
npx ynab-cli-rs --help
```

## Quick Start

```bash
# Authenticate with your YNAB personal access token
ynab auth login --pat

# List your budgets
ynab plans list

# Set a default budget so you don't need --plan-id every time
ynab plans set-default <PLAN_ID>

# List transactions
ynab transactions list
```

## MCP Server

Start the MCP server for AI agent integration:

```bash
ynab mcp
```

Configure in Claude Desktop:

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

## Supported Platforms

- macOS (Intel and Apple Silicon)
- Linux (x64 and ARM64)

## Links

- [GitHub Repository](https://github.com/0xdecaf/ynab-cli)
- [YNAB API Documentation](https://api.ynab.com)
- [Get a Personal Access Token](https://app.ynab.com/settings/developer)
