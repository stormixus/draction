<!-- Parent: ../AGENTS.md -->
<!-- Generated: 2026-05-02 | Updated: 2026-05-02 -->

# draction-lifecycle

## Purpose
Process-lifecycle utilities: single-instance lock, persistent state file, crash recovery hooks, graceful shutdown. Implements the requirements in `SPEC.md` §A "프로세스 라이프사이클": one Draction at a time, recover orphaned runs, write `state.json` with PID/port/last-seen, drain in-flight workflows up to 30 seconds before forcing exit.

## Key Files
| File | Description |
|------|-------------|
| `Cargo.toml` | Small; depends on `anyhow`, `serde`, `serde_json`, `tracing`, plus a file-locking crate |
| `src/lib.rs` | Re-exports the four submodules |
| `src/lock.rs` | `acquire_lock(path)` — exclusive file lock at `~/Draction/.lock`. Errors if another process already holds it |
| `src/state_file.rs` | `read`/`write` for `~/Draction/state.json` (last_seen, port, pid) |
| `src/crash_recovery.rs` | Helpers for marking incomplete runs as failed at startup |
| `src/shutdown.rs` | Tokio-aware shutdown signal handler (SIGINT/SIGTERM) with grace-period drain |

## For AI Agents

### Working In This Directory
- `acquire_lock` is called twice in current code — once in `app-core::bootstrap` and once directly in `draction-desktop`'s Tauri `setup` closure. That redundancy is intentional for the Tauri path (it bootstraps without going through `app-core`). Keep both call sites if you change the API.
- The lock file itself is harmless to delete manually if a stale process never released it; production code should not auto-delete on startup — failing fast is the desired behavior.
- `state_file` is **best-effort, advisory state** — clients tail it for the API port. Do not store secrets or ingest progress here.

### Testing Requirements
- `cargo test -p draction-lifecycle` — `tempfile`-backed lock contention tests.

### Common Patterns
- All public functions take `&Path` arguments — the caller decides where files live; this crate doesn't assume `~/Draction`.
- `anyhow::Result<T>` everywhere; lock errors include the path and OS error code in their context.

## Dependencies

### External
- `anyhow`, `serde`, `serde_json`, `tracing`, OS file-lock primitive

<!-- MANUAL: -->
