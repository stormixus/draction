<!-- Parent: ../AGENTS.md -->
<!-- Generated: 2026-05-02 | Updated: 2026-05-02 -->

# packages

## Purpose
Shared TypeScript packages consumed by JS-side apps (currently the landing site and the future OpenClaw bridge). Mirrors a subset of the Rust domain so frontends can talk to Draction's local HTTP API without re-deriving types.

## Subdirectories
| Directory | Purpose |
|-----------|---------|
| `shared-types/` | Pure TS interfaces mirroring `draction-domain` Rust structs (Rule/Workflow/Run/IngestEvent) (see `shared-types/AGENTS.md`) |
| `api-client/` | `DractionClient` fetch wrapper for the local HTTP API at `127.0.0.1:9400` (see `api-client/AGENTS.md`) |

## For AI Agents

### Working In This Directory
- These packages are **not yet wired into the Tauri desktop app** — that app talks to the Rust core via `tauri::command` invocations, not HTTP. Don't assume the TS client is the canonical access path.
- Both packages are zero-build (`main` and `types` point directly at `src/index.ts`); consumers compile via their own bundler/tsc. Don't add a build step unless a consumer requires `.d.ts` emit.
- The TS types in `shared-types` have **drifted from Rust** — for example `Rule` here has `priority`/`condition`/`then_action`/`created_at`/`updated_at` while the Rust struct uses `order_index`/`when`/`then`. Treat `crates/draction-domain` as authoritative and update TS to match when the discrepancy matters.

### Testing Requirements
- `pnpm -F @draction/shared-types typecheck` and `pnpm -F @draction/api-client typecheck` (each package's `typecheck` script).

### Common Patterns
- Workspace protocol: `api-client` depends on `shared-types` via `"@draction/shared-types": "workspace:*"`.
- ESM only (`"type": "module"`) targeting Node 20+ / modern browsers.

## Dependencies

### Internal
- `api-client` → `shared-types`

### External
- TypeScript 5.7

<!-- MANUAL: -->
