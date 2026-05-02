<!-- Parent: ../AGENTS.md -->
<!-- Generated: 2026-05-02 | Updated: 2026-05-02 -->

# crates

## Purpose
Rust workspace members implementing the Draction core. Layered cleanly: `domain` (types) → `db`/`events`/`inbox` (infrastructure) → `engine` (rule + workflow execution) → `api` (axum HTTP/WS surface) → `app-core` (orchestrates everything for embedding apps). `lifecycle` is a small utility crate used at startup.

## Subdirectories
| Directory | Purpose |
|-----------|---------|
| `draction-domain/` | Pure data types: `Event`, `Rule`, `Workflow`, ID helpers — no I/O (see `draction-domain/AGENTS.md`) |
| `draction-db/` | SQLite schema, migrations, and synchronous `rusqlite` wrappers for events/runs/rules/workflows (see `draction-db/AGENTS.md`) |
| `draction-events/` | In-process `tokio::sync::broadcast` event bus + WS-shaped `Envelope` (see `draction-events/AGENTS.md`) |
| `draction-inbox/` | File-system ingest: move-into-Inbox, sha256/size, undo stack (see `draction-inbox/AGENTS.md`) |
| `draction-engine/` | Rule matcher (Hazel-style condition tree) + workflow executor with pluggable nodes (see `draction-engine/AGENTS.md`) |
| `draction-api/` | axum HTTP + WS server, bearer-token auth, localhost-only middleware (see `draction-api/AGENTS.md`) |
| `draction-app-core/` | `DractionRuntime` — the embeddable façade that the desktop and native shells consume (see `draction-app-core/AGENTS.md`) |
| `draction-lifecycle/` | Process-singleton lock, state file, crash recovery, graceful shutdown (see `draction-lifecycle/AGENTS.md`) |

## For AI Agents

### Working In This Directory
- **Dependency direction is one-way and shallow.** `domain` has no internal deps; `db`/`events`/`inbox` depend only on `domain`; `engine` depends on `domain` (+ `inbox` for executors); `api` depends on `domain`/`db`/`events`; `app-core` depends on everything. Do not introduce cycles.
- All crates use the workspace dependency aliases (`tokio.workspace = true`, `serde.workspace = true`, etc.) — add new shared deps to the root `Cargo.toml` `[workspace.dependencies]`.
- IDs are minted in `draction-domain::ids` (`new_event_id`, `new_run_id`, etc.). Don't generate UUIDs ad-hoc elsewhere.

### Testing Requirements
- `cargo test --workspace` — runs all crate-level tests.
- `cargo clippy --all-targets` — project-configured linter.
- Many crates are intentionally test-light at v0.1; treat new tests as additive infrastructure, not coverage gates.

### Common Patterns
- Library APIs return `anyhow::Result<T>` for now; convert to `thiserror`-typed errors when callers need to branch on failure modes.
- Persistence in `draction-db` is synchronous (`rusqlite`); call it from `tokio::task::spawn_blocking` if hotpath performance ever matters (currently it does not).
- Events flow through a single shared `Arc<EventBus>` constructed in `app-core` / `draction-desktop` and threaded everywhere.

## Dependencies

### External
- See root `Cargo.toml` `[workspace.dependencies]` — axum, tokio, rusqlite (bundled), serde, tracing, anyhow, thiserror, chrono, uuid, sha2, tower-http.

<!-- MANUAL: -->
