# Security

## Reporting a vulnerability

Use GitHub's private vulnerability reporting on this repository
(Security tab, "Report a vulnerability"). Please do not open a public issue
for security reports. You should get an acknowledgement within 7 days.

## Threat model

`ynab-cli` holds a bearer token for your YNAB account. Anyone who can run
the binary as your user, or who can drive the MCP server, can read and
modify your budget.

**Token storage.** Tokens are stored in the OS keychain on macOS. On other
platforms they fall back to a file under the config directory with mode
0600. `ynab auth token` prints the raw token to stdout by design; treat
that output as a secret.

**OAuth.** The OAuth flow uses authorization code with PKCE and a random
`state` value. The redirect listener binds to localhost only.

**MCP server.** The MCP server exposes tools that create, update, delete,
and import transactions and that move budgeted money. An LLM agent driving
the server can perform any of those actions. Run with `ynab mcp --read-only`
(or `YNAB_MCP_READ_ONLY=1`) unless the agent needs write access, and
prefer a dedicated YNAB plan for experimentation.

**Raw API passthrough.** `ynab api` and the `ynab_api_raw` tool forward
arbitrary requests to the YNAB API with your token. In read-only mode only
GET is permitted.

## Supported versions

Only the latest release receives security fixes.
