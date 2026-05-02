<!-- Parent: ../AGENTS.md -->
<!-- Generated: 2026-05-02 | Updated: 2026-05-02 -->

# apps

## Purpose
End-user deliverables. Each subdirectory is a distinct application — two desktop shells over the same Rust core, a public marketing site, and a placeholder for the future OpenClaw bridge. They share types and clients via `packages/` and Rust core via `crates/`.

## Subdirectories
| Directory | Purpose |
|-----------|---------|
| `draction-desktop/` | Primary product: Tauri 2 + React 19 desktop app with overlay + dashboard windows (see `draction-desktop/AGENTS.md`) |
| `draction-native/` | Pure-Rust iced 0.14 reference shell embedding `draction-app-core` directly — used for headless-ish testing of the runtime (see `draction-native/AGENTS.md`) |
| `landing/` | Astro 5 + React marketing site for draction.app (see `landing/AGENTS.md`) |
| `openclaw-bridge/` | Placeholder workspace for the future OpenClaw AI integration described in `SPEC.md` §3 (see `openclaw-bridge/AGENTS.md`) |

## For AI Agents

### Working In This Directory
- Both desktop apps depend on `draction-app-core` (and the Tauri app additionally on `draction-api`, `draction-events`, `draction-inbox`). Changes to the runtime's public API ripple here — update both call sites in lockstep.
- The desktop and native apps independently bootstrap the database and HTTP server. Running them simultaneously will hit the **same `~/Draction/.lock`** and one will fail to start; this is intentional (single-instance policy from SPEC §A).
- Only `draction-desktop`, `landing`, and `openclaw-bridge` are pnpm workspaces. `draction-native` is Rust-only — invoke it with `cargo run -p draction-native` (or `pnpm native` from the root).

### Testing Requirements
- Run `cargo test` from the repo root for any Rust changes.
- For the Tauri app: `pnpm -F draction-desktop dev` for live development; `pnpm build` to verify the production bundle compiles.
- Manual smoke test: drop a file onto the running app's overlay → confirm it lands in `~/Draction/Inbox/<date>/` and a `RUN_FINISHED` event appears.

### Common Patterns
- Each app owns its `bootstrap()` flow but reuses `DractionRuntime::bootstrap()` (or its components) — do not re-implement lock acquisition, DB open, or token loading at the app layer.

## Dependencies

### Internal
- `crates/draction-app-core` — used by both desktop shells
- `crates/draction-api` + `crates/draction-events` + `crates/draction-inbox` + `crates/draction-lifecycle` — used directly by `draction-desktop`'s Tauri layer
- `packages/shared-types` + `packages/api-client` — TS-side mirror of the API/domain (currently consumed only by the landing site / future bridge)

<!-- MANUAL: -->
