# Privacy Policy

**Last Updated: March 15, 2026**

## Overview

ynab-cli is an open-source command-line interface and MCP server for interacting with the YNAB (You Need A Budget) API. This privacy policy explains how your data is handled when you use this application.

## What Data We Access

When you authenticate and use ynab-cli, the application accesses your YNAB account data through the official YNAB API. This may include:

- Budget and plan information
- Account names and balances
- Transactions and transaction details
- Categories and category balances
- Payees and payee information
- Scheduled transactions
- Monthly budget summaries

The application only accesses data that you explicitly request through CLI commands or MCP tool calls.

## How Your Data Is Handled

**ynab-cli is a local application.** Your data is processed entirely on your own device.

- **No servers.** There is no backend server, cloud service, or remote infrastructure. All API calls go directly from your device to the YNAB API.
- **No telemetry.** The application does not collect analytics, usage metrics, crash reports, or any other telemetry.
- **No tracking.** There are no cookies, fingerprinting, or user tracking of any kind.
- **No data storage beyond your device.** Budget data retrieved from the YNAB API is displayed or written to local files at your direction. It is not cached, indexed, or persisted by the application unless you explicitly use the `--output` flag to save results to a file.

## Authentication & Credentials

ynab-cli supports two authentication methods:

1. **Personal Access Tokens (PAT):** Stored locally in your operating system's credential manager (macOS Keychain) or in `~/.config/ynab/credentials.json` with restricted file permissions (`0600`).

2. **OAuth 2.0:** Uses the standard YNAB OAuth authorization code flow. Access tokens and refresh tokens are stored locally in your operating system's credential manager or config directory, same as personal access tokens. The application's OAuth client credentials are embedded in the binary solely for authentication purposes.

Credentials never leave your device except to authenticate with the YNAB API.

## Third-Party Data Sharing

**Your data is never shared with any third party.** ynab-cli communicates exclusively with the official YNAB API (`https://api.ynab.com`). No data is sent to any other service, server, or endpoint.

## Data Security

- Authentication tokens are stored in your operating system's native credential manager (e.g., macOS Keychain) when available, with a JSON file fallback using restrictive file permissions.
- All communication with the YNAB API uses HTTPS/TLS encryption.
- OAuth tokens are automatically refreshed and old tokens are not retained.
- The application is open source — you can audit the code at [github.com/0xdecaf/ynab-cli](https://github.com/0xdecaf/ynab-cli).

## Data Retention

ynab-cli does not retain your budget data. Data retrieved from the YNAB API exists only in memory during command execution and is discarded when the command completes, unless you explicitly write it to a file.

Authentication tokens are stored locally until you run `ynab auth logout`, which removes all stored credentials.

## Your Rights

- **Access:** You can view your stored credentials using `ynab auth status` and `ynab auth token`.
- **Deletion:** Run `ynab auth logout` to remove all stored credentials from your device. Delete any output files you created. Uninstall the binary to fully remove the application.
- **Revocation:** You can revoke the application's access to your YNAB account at any time through your [YNAB account settings](https://app.ynab.com/settings).

## Changes to This Policy

If this privacy policy is updated, the "Last Updated" date at the top will be revised. Significant changes to data handling practices will be communicated through the project's GitHub releases.

## Contact

For privacy-related questions or concerns, please open an issue at [github.com/0xdecaf/ynab-cli/issues](https://github.com/0xdecaf/ynab-cli/issues).
