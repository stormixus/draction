<!-- Parent: ../AGENTS.md -->
<!-- Generated: 2026-05-02 | Updated: 2026-05-02 -->

# src-tauri

## Purpose
The Rust backend of the Tauri desktop app — workspace member `draction-desktop`. Bootstraps lock + DB + auth token + EventBus + API server in `tauri::Builder::default().setup()`, registers Tauri commands, and ships migrations + tray/icon resources.

## Key Files
| File | Description |
|------|-------------|
| `Cargo.toml` | Workspace member; depends on every Draction crate plus `tauri = { version = "2", features = ["macos-private-api"] }` |
| `build.rs` | Tauri build hook (`tauri_build::build()`) generating boilerplate at compile time |
| `tauri.conf.json` | Window definitions (dashboard + overlay), bundle identifier, capabilities allowlist |

## Subdirectories
| Directory | Purpose |
|-----------|---------|
| `src/` | Rust source: `lib.rs` boot + commands + platform shims (see `src/AGENTS.md`) |
| `capabilities/` | Tauri 2 capability manifests — `default.json` enumerates allowed commands per window |
| `migrations/` | SQLite migrations — `001_init.sql` |
| `resources/` | Bundled binaries (e.g. ffmpeg under `bin/macos/`) shipped with the app |
| `icons/` | macOS app icons (32/128/128@2x PNGs) |
| `gen/` | Tauri-generated schemas — gitignored, never edit |

## For AI Agents

### Working In This Directory
- Tauri 2 uses **capability-based command exposure**. A new `#[tauri::command]` won't be callable from JS until you add it to `invoke_handler![...]` AND grant access in `capabilities/default.json`.
- `setup()` runs **before** the API server is ready — `ApiPort` is `manage()`d after a `tauri::async_runtime::spawn` task completes. The `get_api_port` command must handle the brief startup window where the resource is `ApiPort(0)`.
- Ship binaries (ffmpeg etc.) live in `resources/bin/<platform>/` and are referenced from executors via `tauri::path::resolve_path`. Adding a new bundled binary requires updating `tauri.conf.json` `bundle.resources`.
- Migration files are read at runtime by `draction-db` from a path passed via `app-core` — they are **not** embedded in the Rust binary. Make sure they ship in the bundle.

### Testing Requirements
- `cargo build -p draction-desktop` — verify the Tauri side compiles.
- `pnpm -F draction-desktop tauri dev` — full dev experience (slow first build, run in background).

### Common Patterns
- All Tauri commands return `Result<T, String>` (Tauri serializes the error string to JS-side `Promise.reject`).
- Shared state via `app.manage(...)` + `tauri::State<'_, T>` extractor.

## Dependencies

### Internal
- All `crates/draction-*`

### External
- Tauri 2.10, `tauri-build` 2, `dirs` 6, plus the workspace's standard async/serde/tracing trio

<!-- MANUAL: -->
