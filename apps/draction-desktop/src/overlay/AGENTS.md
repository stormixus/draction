<!-- Parent: ../AGENTS.md -->
<!-- Generated: 2026-05-02 | Updated: 2026-05-02 -->

# overlay

## Purpose
The transparent always-on-top drop-zone window. Renders the Draky character (sprite/Lottie), highlights on drag-over, and forwards file drops to the Rust `ingest_files` Tauri command. Mounted at `overlay.html` as a separate Vite entry from the dashboard.

## Key Files
| File | Description |
|------|-------------|
| `App.tsx` | Overlay React root — drag/drop visual feedback + animation states (idle/hover/eating/success/fail) |
| `main.tsx` | Vite/React entry mounted by `overlay.html` |

## For AI Agents

### Working In This Directory
- The overlay window is **created with `transparent: true` + `decorations: false`** in Tauri config, so the React tree must paint its own background only where it wants pixels (the rest must be alpha=0). Adding a full-screen `bg-*` will fill the whole desktop.
- Click-through is toggled via the `set_overlay_visible` / platform shim (`src-tauri/src/platform/macos.rs`). Don't try to manage it from JS.
- File drops arrive via Tauri's webview drop event, **not** HTML5 drag-drop — listen to `getCurrent().listen("tauri://file-drop", ...)` from `@tauri-apps/api`.

### Testing Requirements
- Manual visual smoke test only; no automated tests.

### Common Patterns
- framer-motion for the bounce/glow states; lottie-react for character animations.
- Keep state local — the overlay window owns its own animation state independent of the dashboard.

## Dependencies

### External
- React 19, framer-motion, lottie-react, `@tauri-apps/api`

<!-- MANUAL: -->
