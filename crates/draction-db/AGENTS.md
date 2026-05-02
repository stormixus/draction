<!-- Parent: ../AGENTS.md -->
<!-- Generated: 2026-05-02 | Updated: 2026-05-02 -->

# draction-db

## Purpose
SQLite persistence layer wrapping `rusqlite` (bundled). Owns the schema for events, runs, rules, workflows. Exposes `DractionDb` (synchronous, connection-per-call style) and row structs (`EventRow`, `RuleRow`, `RunRow`, `WorkflowRow`).

## Key Files
| File | Description |
|------|-------------|
| `Cargo.toml` | Depends on `rusqlite` (bundled), `serde`, `chrono`, `anyhow`, `draction-domain` |
| `src/lib.rs` | Re-exports `DractionDb`, row structs, `Database` (pool), `Repository` |
| `src/db.rs` | `DractionDb` struct — open, migrate, CRUD methods (`insert_event`, `insert_run`, `update_run_status`, `list_runs`, `mark_running_as_failed`) |
| `src/pool.rs` | `Database` connection pool wrapper (currently a single connection) |
| `src/repo.rs` | `Repository` trait abstraction (early scaffolding) |

## For AI Agents

### Working In This Directory
- Schema lives in **`apps/draction-desktop/src-tauri/migrations/001_init.sql`**, not in this crate. The DB module reads/applies migrations from a path passed in at open time. Update both this code and the migration file together when changing tables.
- API is **synchronous** because `rusqlite` is. Callers wrap in `spawn_blocking` if needed; at v0.1 latencies are low enough that we don't.
- `mark_running_as_failed()` is called at startup by `app-core` to clean up runs left in `running` state from a previous crash (`SPEC.md` §A "충돌 복구").
- `list_runs(status, limit)` filters by status when `Some`; pass `None` for all.

### Testing Requirements
- `cargo test -p draction-db` — uses `:memory:` SQLite for unit tests where applicable.

### Common Patterns
- All public methods return `anyhow::Result<T>`.
- Timestamps stored as RFC3339 strings (`chrono::Utc::now().to_rfc3339()`), not as integers.
- Foreign keys are declared but enforcement depends on `PRAGMA foreign_keys = ON` set at connection open.

## Dependencies

### Internal
- `draction-domain` (for `EventRow`/`RuleRow` shape conversions)

### External
- `rusqlite` (bundled SQLite — no system dep), `serde`, `chrono`, `anyhow`

<!-- MANUAL: -->
