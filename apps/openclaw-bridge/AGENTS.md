<!-- Parent: ../AGENTS.md -->
<!-- Generated: 2026-05-02 | Updated: 2026-05-02 -->

# openclaw-bridge

## Purpose
Placeholder for the v0.3 OpenClaw integration described in `SPEC.md` §3 — a TypeScript module that will subscribe to Draction's local WebSocket, surface events in OpenClaw's UI, and POST AI-generated rules back via the local HTTP API. Currently empty (only an `src/` directory exists in the workspace).

## Subdirectories
| Directory | Purpose |
|-----------|---------|
| `src/` | (empty) — future home of the bridge client |

## For AI Agents

### Working In This Directory
- This workspace is **declared in `pnpm-workspace.yaml` but has no `package.json` yet**. Adding files here without a manifest will break workspace resolution; create a proper `package.json` first if you start implementing.
- When implemented, this crate should consume `@draction/api-client` and `@draction/shared-types` from `packages/` rather than re-deriving HTTP/WS shapes.
- Authentication flow per `SPEC.md` §3: pair once via `POST /api/pair` to receive the bearer token, then store it in OpenClaw's own config.

### Common Patterns
- v0.1 has no code here. Don't generate scaffolding speculatively — wait for the bridge implementation task.

## Dependencies

### Internal (planned)
- `packages/api-client` (HTTP)
- `packages/shared-types` (types)

<!-- MANUAL: -->
