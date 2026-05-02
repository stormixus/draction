<!-- Parent: ../AGENTS.md -->
<!-- Generated: 2026-05-02 | Updated: 2026-05-02 -->

# commands

## Purpose
Tauri commands — the typed RPC surface the React frontend calls via `invoke()`. Each file maps to one logical concern.

## Key Files
| File | Description |
|------|-------------|
| `mod.rs` | Re-exports the four command modules |
| `ingest.rs` | `ingest_files(paths: Vec<String>)` — runs the full ingest pipeline (move into Inbox, hash, emit event, run rules), records undo entry |
| `overlay.rs` | `set_overlay_visible(visible: bool)` — toggles overlay window visibility + click-through |
| `settings.rs` | `get_api_port()` — returns the dynamically chosen API port for the frontend to build URLs |
| `undo.rs` | `undo_last_ingest()` — pops the head of `AppUndoStack` and reverses the move/copy. `AppUndoStack` is the Tauri-managed wrapper around `draction-inbox::undo::UndoStack` |

## For AI Agents

### Working In This Directory
- Every new command must be added to the `invoke_handler![...]` macro in `lib.rs` AND the capability list in `capabilities/default.json`. Forgetting either causes silent rejection at the JS boundary.
- Commands take `tauri::State<'_, T>` for managed dependencies (UndoStack, AppState). Don't reach into globals.
- Long-running commands should `async fn`; Tauri runs them on its async runtime so they don't block the webview.
- Return `Result<T, String>` — the error string lands in JS as a `Promise` rejection.

### Testing Requirements
- Compile-only checks via `cargo build -p draction-desktop`. Runtime testing happens in the dev shell.

### Common Patterns
- Wrap shared state in a tuple struct (e.g. `pub struct AppUndoStack(pub Arc<Mutex<UndoStack>>);`) so Tauri's typeid-based state lookup is unambiguous.
- Convert `anyhow::Error` to `String` at the boundary with `.map_err(|e| e.to_string())`.

## Dependencies

### Internal
- `draction-inbox` (UndoStack), `draction-events`, `draction-app-core` indirectly through the API server's AppState

<!-- MANUAL: -->
