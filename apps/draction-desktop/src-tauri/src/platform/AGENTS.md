<!-- Parent: ../AGENTS.md -->
<!-- Generated: 2026-05-02 | Updated: 2026-05-02 -->

# platform

## Purpose
OS-specific implementations behind a small cross-platform façade. v0.1 only needs macOS click-through manipulation for the overlay window; Windows and Linux callers get no-op stubs.

## Key Files
| File | Description |
|------|-------------|
| `mod.rs` | Public façade — `set_overlay_click_through(window, click_through)` with `#[cfg(target_os = "macos")]` dispatch |
| `macos.rs` | macOS implementation using `objc2` / Cocoa interop to set `NSWindow.ignoresMouseEvents` |

## For AI Agents

### Working In This Directory
- All platform-specific logic must be gated behind `#[cfg(target_os = "...")]`. The façade in `mod.rs` is the only thing other modules import — they should not `#[cfg]` themselves.
- macOS implementations rely on `tauri = { features = ["macos-private-api"] }` (set in `Cargo.toml`). Don't remove that feature without finding alternative APIs.
- When adding Windows/Linux support, mirror the existing `mod.rs` pattern (one file per OS, façade dispatches).

### Testing Requirements
- Cross-platform compilation only verifiable on the target host — `cargo check` on each OS.

### Common Patterns
- Functions in the façade take `&tauri::WebviewWindow` first; OS modules can extract the native handle via `window.ns_window()` etc.

<!-- MANUAL: -->
