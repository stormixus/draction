<!-- Parent: ../AGENTS.md -->
<!-- Generated: 2026-05-02 | Updated: 2026-05-02 -->

# draction-app-core

## Purpose
The embeddable façade. `DractionRuntime` owns the bootstrap sequence (lock → DB → token → event bus → API server → default rules/workflows), then exposes the high-level operations that frontends actually call: `ingest_paths`, `list_runs`, `list_rules`. Both the Tauri desktop crate and the iced native shell consume this crate so the same runtime logic lives in one place.

## Key Files
| File | Description |
|------|-------------|
| `Cargo.toml` | Depends on every other Draction crate plus `dirs`, `chrono`, `anyhow`, `tracing`, `serde`, `serde_json` |
| `src/lib.rs` | `DractionRuntime` struct + `bootstrap()` + `ingest_paths()` (the full drop pipeline) + default-rules/workflows seeding |

## For AI Agents

### Working In This Directory
- `bootstrap()` is the **only** sanctioned bring-up path — it acquires the lock, opens the DB, marks orphaned runs as failed, loads/creates the auth token, starts the API server, and seeds defaults. Don't call those steps individually from another crate.
- `ingest_paths()` walks the dropped paths recursively, ingests each file into the Inbox, emits `EVENT_INGESTED`, runs the first matching rule's workflow, and emits `RUN_STARTED`/`RUN_FINISHED`/`RUN_FAILED`. It writes everything to the DB along the way.
- Default rules + workflows are written to **`~/Draction/rules.json`** and **`~/Draction/workflows.json`** on first run. They cover the obvious "Pictures → Photos / Videos → Videos / Documents → Documents" buckets. After first run, those files are user-editable and **never overwritten**.
- The runtime is `Clone` (cheap — wraps `Arc`s). Pass clones into async tasks rather than holding the original.

### Testing Requirements
- `cargo test -p draction-app-core` — at v0.1 mostly bootstrap smoke tests.

### Common Patterns
- All public methods return `anyhow::Result<T>` and log via `tracing` on entry/exit when useful.
- `RunSummary` / `RuleSummary` / `IngestResult` are the cross-language friendly shapes (also serialized to JSON for Tauri events).
- Workflow execution failures are caught and recorded — the function returns `Ok` with the failure embedded in `IngestResult.action`. It does not bubble per-file errors.

## Dependencies

### Internal
- All other crates: `draction-domain`, `draction-db`, `draction-events`, `draction-inbox`, `draction-engine`, `draction-api`, `draction-lifecycle`

### External
- `dirs`, `chrono`, `anyhow`, `tracing`, `serde`, `serde_json`

<!-- MANUAL: -->
