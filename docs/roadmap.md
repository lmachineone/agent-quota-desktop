# Roadmap

## Metadata

- CREATED_AT: `2026-06-21T06:52:47Z`
- UPDATED_AT: `2026-06-21T06:52:47Z`

## Now

- Create a Rust workspace with core and CLI crates.
- Lock the public security boundary before adding provider auth paths.
- Implement the shared usage model and first fixture-backed parser tests.
- Build the first CLI slice for a small provider wedge: Codex, Claude, and OpenAI.

## Next

- Add provider fetch traits, source-mode selection, timeout policy, and redacted diagnostics.
- Add config-file discovery for XDG, macOS Application Support, and Windows AppData.
- Prototype the tray shell with dynamic icon updates, manual refresh, and one provider detail card.

## Later

- Browser-cookie import, OAuth/device flows, and OS keychain adapters.
- Multi-account management.
- Provider status polling.
- Cost-history scans.
- Signed release packaging and auto-update lanes.

## Noise

- Porting every CodexBar provider before the provider contract is proven.
- Pixel-copying macOS-specific SwiftUI/AppKit details before Windows and Linux tray behavior works.
- Public sample configs containing real-looking tokens, cookies, or browser-session material.
