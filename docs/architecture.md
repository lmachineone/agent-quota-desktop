# Architecture

## Metadata

- CREATED_AT: `2026-06-21T06:52:47Z`
- UPDATED_AT: `2026-06-21T06:52:47Z`

## Principle

The provider engine is the product moat. The desktop tray shell is the distribution surface.

## Planned Shape

- `agent-quota-core`: provider contracts, shared usage models, reset-window math, pacing, redaction, and cache policy.
- `agent-quota-cli`: cross-platform CLI for scripts, status bars, CI, and desktop shell IPC.
- `apps/desktop`: tray shell for macOS, Linux, and Windows.
- `docs`: public product, security, and roadmap decisions.

## Provider Contract

Each provider should declare:

- stable provider ID
- display metadata
- supported source modes: API, OAuth, CLI, web, local
- timeout-bounded fetch strategies
- privacy and credential requirements
- fixture-backed parser tests

## Security Boundary

Provider credentials are never committed. Public docs must use placeholders only. Debug exports must be redacted before
they can be attached to issues.
