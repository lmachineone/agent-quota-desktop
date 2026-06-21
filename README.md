# agent-quota-desktop

## Metadata

- CREATED_AT: `2026-06-21T06:52:47Z`
- UPDATED_AT: `2026-06-21T06:52:47Z`

Cross-platform AI agent quota, credit, spend, and reset control plane for macOS, Linux, and Windows.

## Positioning

Every agent quota. Every desktop. One control plane.

This repository is the clean-room product home for a Rust-first desktop and CLI implementation inspired by the public
CodexBar product category. The upstream reference fork lives at
[`lmachineone/codexbar-upstream-reference`](https://github.com/lmachineone/codexbar-upstream-reference).

## Scope

- Rust provider core for quota, credit, spend, reset-window, status, and cost-scan models.
- Cross-platform CLI first, then a tray desktop shell for macOS, Linux, and Windows.
- Public-safe development: no real cookies, API tokens, OAuth credentials, browser session dumps, or local provider
  configs belong in this repository.

## Current Status

Bootstrap only. The first real milestone is a narrow CLI slice with provider contracts and fixture-backed tests before
the desktop tray shell expands.

## Security

Read [SECURITY.md](SECURITY.md) before opening issues or pull requests that involve provider auth, browser cookies,
OAuth, keychains, local config files, logs, screenshots, or diagnostics.
