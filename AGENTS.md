<!-- Generated: 2026-05-02 | Updated: 2026-05-02 -->

# draction

## Purpose
Draction is a desktop file-ingest companion: a floating overlay that accepts dropped files, moves them to a managed Inbox, evaluates Hazel-style rules, and runs n8n-lite workflows (move, copy, rename, transcode, webhook). The codebase is a hybrid pnpm + Cargo monorepo with a Rust core (axum HTTP/WS API, SQLite persistence, workflow engine) and two desktop frontends (Tauri/React and an iced-based native shell). Future versions plan to integrate an "OpenClaw" AI peer (see `SPEC.md`).

## Key Files
| File | Description |
|------|-------------|
| `Cargo.toml` | Rust workspace root listing all crates and shared dependencies (axum 0.8, tokio, rusqlite, serde, tracing) |
| `package.json` | Top-level pnpm scripts: `dev`/`build` (Tauri desktop), `tauri`, `native` (iced app) |
| `pnpm-workspace.yaml` | JS workspaces: `apps/draction-desktop`, `apps/landing`, `apps/openclaw-bridge`, `packages/*` |
| `Cargo.lock` | Pinned Rust dependency graph |
| `pnpm-lock.yaml` | Pinned JS dependency graph |
| `SPEC.md` | v0.1 product/architecture specification (Korean) — drop UX, API contract, rule/workflow schemas, safety policy |
| `README.md` | High-level overview, MVP feature list, ingest-pipeline summary, roadmap |
| `.gitignore` | Excludes `target/`, `node_modules/`, `dist/`, `.venv/`, IDE state |

## Subdirectories
| Directory | Purpose |
|-----------|---------|
| `apps/` | Deliverable applications: Tauri desktop, iced native, Astro landing, OpenClaw bridge stub (see `apps/AGENTS.md`) |
| `crates/` | Rust workspace crates implementing domain, persistence, engine, API, lifecycle (see `crates/AGENTS.md`) |
| `packages/` | Shared TypeScript packages: API client + domain types (see `packages/AGENTS.md`) |
| `docs/` | Architecture docs, ADRs, design notes, and image assets (see `docs/AGENTS.md`) |
| `scripts/` | Python helper scripts for sprite/asset processing (see `scripts/AGENTS.md`) |
| `.claude/`, `.omc/` | Claude Code / oh-my-claudecode runtime state — do not edit by hand |
| `target/`, `node_modules/`, `dist/` | Build outputs — never commit, never edit |

## For AI Agents

### Working In This Directory
- This is a **dual-toolchain** repo. Use `cargo` commands for Rust crates and `pnpm` for JS/TS packages. Both must build cleanly.
- The Rust workspace is the single source of truth for domain types — TypeScript `packages/shared-types` mirrors the Rust structs and **drifts easily**; verify field names match `crates/draction-domain/src/` before relying on the TS types.
- The user's runtime state lives at `~/Draction/` (db, lock, config, rules.json, workflows.json, Inbox/). Do **not** assume it exists in CI; `DractionRuntime::bootstrap()` creates it lazily.
- Two frontends share the Rust core: the **Tauri** app (`apps/draction-desktop`) talks via `tauri::command` invocations, while the **native** app (`apps/draction-native`) embeds `draction-app-core` directly. Keep both invocation paths working when changing the runtime API.

### Testing Requirements
- `cargo test` from the workspace root runs all crate tests.
- `cargo clippy --all-targets` is the configured lint command.
- `pnpm build` builds the Tauri desktop app (the only JS app wired into the root `build` script). Other workspaces have their own `build`/`typecheck` scripts.
- There is no end-to-end test harness yet; manual ingest verification is done by dropping files onto the running app.

### Common Patterns
- All IDs are stringly typed (`evt_*`, `run_*`, `rule_*`, `wf_*`) generated in `draction-domain::ids`.
- Errors propagate as `anyhow::Result<T>` in app/runtime layers; library crates use `thiserror` where domain-specific errors matter.
- Async Rust is `tokio` everywhere; database access goes through the synchronous `rusqlite` `DractionDb` wrapper.
- Events flow through a single in-process `tokio::sync::broadcast` `EventBus` (`draction-events`) and are mirrored to WebSocket clients in `draction-api::ws`.
- Rules and workflows persist as **plain JSON files** at `~/Draction/rules.json` and `~/Draction/workflows.json` (defaults seeded on first run); SQLite holds events, runs, and audit data only.

## Dependencies

### External
- **Rust**: axum 0.8 (HTTP + WS), tokio (async), rusqlite (bundled SQLite), serde/serde_json, tracing, anyhow/thiserror, chrono, uuid, sha2, tower-http
- **Tauri 2.x** with `macos-private-api` feature for the desktop overlay
- **iced 0.14** for the native cross-platform shell
- **React 19** + **Vite 6** + **Tailwind 4** + **framer-motion** for the Tauri frontend
- **Astro 5** + React for the landing site
- **pnpm** workspaces (manifest declares `pkg:pnpm`)

<!-- MANUAL: -->
