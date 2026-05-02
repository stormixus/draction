<!-- Parent: ../AGENTS.md -->
<!-- Generated: 2026-05-02 | Updated: 2026-05-02 -->

# src

## Purpose
Rust source for the Tauri backend: top-level `run()` bootstraps the Tauri builder, registers commands, kicks off the API server. Helper modules under `commands/` and `platform/`.

## Key Files
| File | Description |
|------|-------------|
| `main.rs` | Tiny entry — calls `draction_desktop_lib::run()` |
| `lib.rs` | The actual app: tracing init, Tauri `Builder` setup, lock acquisition, DB open, auth token load, EventBus + AppState construction, API server spawn, command registration |

## Subdirectories
| Directory | Purpose |
|-----------|---------|
| `commands/` | `#[tauri::command]` functions exposed to JS (`ingest_files`, `set_overlay_visible`, `get_api_port`, `undo_last_ingest`) (see `commands/AGENTS.md`) |
| `platform/` | OS-specific shims, currently just macOS click-through (see `platform/AGENTS.md`) |

## For AI Agents

### Working In This Directory
- `lib.rs` is structured as: `setup()` for synchronous prep + `tauri::async_runtime::spawn` for the API server (so the UI doesn't block on its startup). Don't await server bootstrap from `setup`.
- `AppState` is constructed once and `manage()`d on the Tauri handle so all commands can pull it via `State<'_, AppState>`.
- The `greet` command is leftover scaffolding from the Tauri starter — safe to keep, safe to remove.

### Common Patterns
- `tracing_subscriber::fmt().with_env_filter(EnvFilter::from_default_env()).init()` so `RUST_LOG=draction=debug` works.
- Errors in `setup()` are converted to `Box<dyn std::error::Error>` via `.map_err(|e| format!(...))?`.

<!-- MANUAL: -->
