# Security Policy

## Metadata

- CREATED_AT: `2026-06-21T06:52:47Z`
- UPDATED_AT: `2026-06-21T06:52:47Z`

## Public Repository Rule

Do not commit, paste, upload, or attach:

- API keys, OAuth tokens, refresh tokens, device-flow secrets, or bearer tokens
- browser cookies, cookie headers, browser session exports, Safe Storage keys, or keychain dumps
- real local config files from providers such as Codex, Claude, Cursor, Copilot, OpenAI, or similar tools
- screenshots that expose accounts, org IDs, billing IDs, request headers, cookies, or tokens
- raw logs before redaction

## Issue Reports

Use placeholders such as `<REDACTED_TOKEN>` and `<ACCOUNT_EMAIL_REDACTED>`. If a report requires sensitive material,
open a minimal public issue that describes the failure class and coordinate a private transfer path with maintainers.

## Diagnostics

Diagnostics must default to redaction. Any future diagnostic export command must document exactly which fields are
collected, which fields are redacted, and which fields are intentionally omitted.

## Supported Versions

No release is supported yet. This repository is in bootstrap.
