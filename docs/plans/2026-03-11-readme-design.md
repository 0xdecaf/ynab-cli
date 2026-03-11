# README & CI Improvements Design

## Date: 2026-03-11

## Overview

Create a professional-grade README.md, add code coverage to CI, and add MIT LICENSE file.

## Badges

CI status · Codecov · npm version · GitHub Release · License (MIT)

## README Structure

1. **Hero** — one-liner + hook: only YNAB CLI with built-in MCP server
2. **Features** — emoji bullet grid (MCP, output formats, dollar mode, search, field filtering, single binary, secure auth, delta sync)
3. **Install** — 3 methods: npm, GitHub Release curl, build from source
4. **Quick Start** — auth → list plans → set default → list transactions
5. **Usage** — grouped command reference with examples
6. **MCP Server** — config snippet, tool count, why it matters
7. **Contributing** — clone, build, test, PR
8. **License** — MIT

## CI Changes

Add `cargo-tarpaulin` coverage job, upload to Codecov.

## New Files

- `LICENSE` (MIT, 2026, 0xdecaf)
- `README.md`
- Update `.github/workflows/ci.yml` with coverage job
