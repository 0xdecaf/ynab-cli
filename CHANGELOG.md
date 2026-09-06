# Changelog

All notable changes to this project are documented here. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

## [Unreleased]

### Fixed
- `ynab api` and the `ynab_api_raw` MCP tool no longer double the `/v1`
  segment; the prefix is now optional.
- `money-movements by-month` and `groups-by-month` now call the
  month-scoped endpoints instead of ignoring `--month`.
- `rust-version` corrected to 1.88 (the code uses let-chains).

### Added
- `ynab mcp --read-only` (`YNAB_MCP_READ_ONLY`) disables every mutating
  tool and restricts raw requests to GET.
- `SECURITY.md` with a disclosure path and threat model.
- 30-second HTTP timeout on all API requests.
- Mock-server test suite for the client and MCP guard.
- MSRV, `cargo audit`, and `cargo doc` CI jobs; Dependabot config.

### Removed
- The `--verbose` flag, which was accepted but never did anything.

## [0.2.0] - 2026-03-16

### Added
- OAuth2 login with PKCE and automatic token refresh.
- Fallback to the `default` plan ID when none is configured.
- Linux builds switched to musl for portability.

## [0.1.1] - 2026-03-10

### Added
- npm wrapper package `ynab-cli-rs` published on release.
- Feature parity commands: mutations, search, raw API, output options.

## [0.1.0] - 2026-03-10

- Initial release: CLI and MCP server for the YNAB API.
