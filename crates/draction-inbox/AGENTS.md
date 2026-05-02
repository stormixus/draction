<!-- Parent: ../AGENTS.md -->
<!-- Generated: 2026-05-02 | Updated: 2026-05-02 -->

# draction-inbox

## Purpose
Filesystem ingest layer. Computes `~/Draction/Inbox/<YYYY-MM-DD>/` paths, moves (or copies) dropped files into the Inbox, computes `sha256` + size, and maintains the per-process undo stack so the user has the 10-second "oopsie" window described in `SPEC.md` §1.1 / §7.

## Key Files
| File | Description |
|------|-------------|
| `Cargo.toml` | Depends on `tokio`, `sha2`, `chrono`, `anyhow` |
| `src/lib.rs` | Module declarations (`ingest`, `file_ops`, `undo`) |
| `src/ingest.rs` | `inbox_dir(base)` → `<base>/Inbox`; `ingest_file(src, inbox, move_mode)` → moves/copies into `Inbox/<date>/` and returns the new path |
| `src/file_ops.rs` | `compute_sha256(path)`, `file_size(path)`, helpers for safe move with collision-rename |
| `src/undo.rs` | `UndoStack` — bounded LRU (capacity 5 per `SPEC.md` §7) with timestamped entries for rollback |

## For AI Agents

### Working In This Directory
- The default ingest mode is **move** (`ingest_file(.., true)`). Copy mode exists but the desktop app currently always moves.
- Collision handling: if `<inbox>/<date>/<name>` exists, `ingest_file` rewrites the destination as `<name> (n).<ext>` until free. Don't change this without checking the Tauri ingest command.
- Undo entries are pushed by callers (`app-core`/Tauri commands) — this crate doesn't observe ingests automatically.
- SHA-256 is streamed (chunked read) to handle large drops without loading the whole file into memory.

### Testing Requirements
- `cargo test -p draction-inbox` — round-trip tests against `tempfile`-managed temp dirs.

### Common Patterns
- All async functions accept `&Path`/`PathBuf` — never `&str`.
- Return `anyhow::Result<PathBuf>` for the new resting location.
- Date partitioning uses local timezone (`chrono::Local::now()`).

## Dependencies

### External
- `tokio` (fs), `sha2`, `chrono`, `anyhow`

<!-- MANUAL: -->
