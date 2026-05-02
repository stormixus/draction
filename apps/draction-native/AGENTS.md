<!-- Parent: ../AGENTS.md -->
<!-- Generated: 2026-05-02 | Updated: 2026-05-02 -->

# draction-native

## Purpose
A pure-Rust desktop shell built on **iced 0.14** that embeds `draction-app-core` directly. It serves two roles: (1) a no-Tauri reference implementation for testing the runtime end-to-end without web tooling, and (2) a minimal cross-platform fallback. Drag-drop comes from the iced window event stream; everything else (DB, API, ingest, workflow execution) is delegated to `DractionRuntime`.

## Key Files
| File | Description |
|------|-------------|
| `Cargo.toml` | Workspace member; depends only on `draction-app-core` + `iced` + `anyhow` + `tracing` |
| `src/main.rs` | Single-file iced application: `boot`/`update`/`view`/`subscription` + `NativeApp` state struct, `Message` enum, Runs/Rules tabs |

## For AI Agents

### Working In This Directory
- This crate **does not** depend on `draction-api`, `draction-events`, etc. directly. All access is through `DractionRuntime` — keep it that way to avoid duplicate bootstrap logic.
- Drag-drop arrives via `iced::window::Event::FileDropped(path)` in the window event subscription. Multi-file drops produce one event per path.
- The HTTP API still starts on port 9400 (delegated by `bootstrap()`); running this app while the Tauri app is running will fail at the `~/Draction/.lock` step — that is intentional.
- `ingest_paths` is called on the tokio runtime spawned by iced; do not block on it from `update`.

### Testing Requirements
- `cargo run -p draction-native` (or `pnpm native` from repo root) — manual smoke test by dropping files on the window.
- `cargo test -p draction-native` — currently no tests defined.

### Common Patterns
- iced 0.14 functional API (`iced::application(boot, update, view)`).
- All Tasks return `Message` variants; never `unwrap` inside async closures — convert errors to strings with `.map_err(|e| e.to_string())` to keep `Message` `Clone`-able.
- Theme is hard-coded to `Theme::TokyoNight`.

## Dependencies

### Internal
- `crates/draction-app-core`

### External
- `iced` 0.14 with `tokio` feature

<!-- MANUAL: -->
