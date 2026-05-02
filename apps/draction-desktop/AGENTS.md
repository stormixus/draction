<!-- Parent: ../AGENTS.md -->
<!-- Generated: 2026-05-02 | Updated: 2026-05-02 -->

# draction-desktop

## Purpose
The flagship product: a Tauri 2 desktop application bundling a transparent always-on-top **overlay window** (the drop target) and a regular **dashboard window** (Runs/Rules tabs). The frontend is React 19 + Vite 6 + Tailwind 4; the backend is the `src-tauri/` Rust crate that wraps the entire `crates/draction-*` stack and exposes `tauri::command` invocations to the UI plus the local HTTP API for external clients (future OpenClaw).

## Key Files
| File | Description |
|------|-------------|
| `package.json` | Frontend scripts: `dev:fe` (vite), `build:fe` (tsc + vite build), `tauri` |
| `index.html` | Vite entry for the dashboard window |
| `overlay.html` | Vite entry for the transparent overlay window — separate Vite multi-page entry |
| `vite.config.ts` | Vite config with multi-page setup and Tailwind plugin |
| `tsconfig.json` | TypeScript project config |

## Subdirectories
| Directory | Purpose |
|-----------|---------|
| `src/` | React frontend code split between dashboard and overlay entry points (see `src/AGENTS.md`) |
| `src-tauri/` | Rust backend crate `draction-desktop` — Tauri commands, platform shims, migrations, resources (see `src-tauri/AGENTS.md`) |
| `public/` | Static assets served by Vite — sprite sheets for the Draky character |
| `dist/` | Vite build output — gitignored, never edit |

## For AI Agents

### Working In This Directory
- This is a **multi-window** Tauri app: the dashboard and the overlay are separate webview windows backed by separate HTML entry points (`index.html`, `overlay.html`) and separate React roots (`src/main.tsx`, `src/overlay/main.tsx`).
- The overlay uses macOS-private-api features (`tauri = { version = "2", features = ["macos-private-api"] }`) and platform-specific click-through code in `src-tauri/src/platform/macos.rs` — Windows/Linux paths are stubbed.
- Frontend talks to the Rust core two ways: (1) Tauri `invoke()` for actions (`ingest_files`, `set_overlay_visible`, `undo_last_ingest`, `get_api_port`) and (2) plain `fetch` against `http://127.0.0.1:<port>/api/v1/*` for read endpoints — see `src/App.tsx`.
- Build runs `tsc -b && vite build` — TypeScript errors are blocking. Don't suppress them with `// @ts-ignore`; fix or refactor.

### Testing Requirements
- `pnpm -F draction-desktop dev` for live development against a debug Tauri binary.
- `pnpm -F draction-desktop tauri build` for a production bundle (slow — run in background if you need it).
- No automated UI tests; verify by dropping a real file onto the overlay and checking `~/Draction/Inbox/<date>/`.

### Common Patterns
- The frontend stays UI-only — all filesystem and DB work crosses into Rust via Tauri commands.
- `App.tsx` polls `/api/v1/runs?limit=1` every 5s as a connectivity heartbeat (drives the green/red status dot).
- React 19 + Tailwind v4 (no separate `tailwind.config.js` — config lives in `@tailwindcss/vite`).

## Dependencies

### Internal
- `crates/draction-domain`, `draction-db`, `draction-events`, `draction-inbox`, `draction-engine`, `draction-api`, `draction-lifecycle` — all linked from `src-tauri/Cargo.toml`

### External
- Tauri 2.10, React 19, Vite 6, Tailwind 4, framer-motion 11, lottie-react, zustand, react-router-dom 7

<!-- MANUAL: -->
