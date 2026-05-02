<!-- Parent: ../AGENTS.md -->
<!-- Generated: 2026-05-02 | Updated: 2026-05-02 -->

# src

## Purpose
Frontend code for both Tauri webviews. The dashboard window mounts `App.tsx` (Runs/Rules tabs); the overlay window mounts `overlay/App.tsx` (transparent drop target with Draky animation). Tailwind v4 + React 19.

## Key Files
| File | Description |
|------|-------------|
| `App.tsx` | Dashboard root — tab switcher, API connectivity heartbeat (polls `/api/v1/runs?limit=1`), invokes `get_api_port` on boot |
| `main.tsx` | Vite/React entry for `index.html` (dashboard) |
| `styles.css` | Global Tailwind imports + custom CSS variables shared by both windows |

## Subdirectories
| Directory | Purpose |
|-----------|---------|
| `components/` | Shared dashboard components: `RunsPanel`, `RulesPanel` (see `components/AGENTS.md`) |
| `dashboard/` | Reserved for dashboard-specific pages/components — currently empty (see `dashboard/AGENTS.md`) |
| `overlay/` | Overlay window root + entry (see `overlay/AGENTS.md`) |

## For AI Agents

### Working In This Directory
- Two React roots, two HTML files, two `main.tsx`. The Vite multi-page setup in `vite.config.ts` glues them. Adding a new top-level window means adding the HTML file and a corresponding `main.tsx`.
- Frontend should remain UI-only. Routing for actions: `import { invoke } from "@tauri-apps/api/core"` and call into Rust commands rather than touching `fetch` for write operations. Reads are fine over HTTP.
- The dashboard's API base URL is **dynamically discovered** by calling `invoke<number>("get_api_port")` (because the server port-walks). Don't hardcode 9400.

### Testing Requirements
- `pnpm -F draction-desktop dev:fe` — Vite dev server only (no Tauri shell).
- Type-check via the build script: `pnpm -F draction-desktop build:fe` (= `tsc -b && vite build`).

### Common Patterns
- Dark theme by default (`bg-zinc-950 text-zinc-100`). Match the existing palette before adding ad-hoc colors.
- React 19 — use the new `use()` hook for promise unwrapping where appropriate; avoid stale `useEffect` patterns.

## Dependencies

### External
- React 19, framer-motion, lottie-react, zustand, react-router-dom 7, `@tauri-apps/api` 2, `@tauri-apps/plugin-shell`

<!-- MANUAL: -->
