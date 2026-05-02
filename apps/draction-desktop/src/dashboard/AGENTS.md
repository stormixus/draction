<!-- Parent: ../AGENTS.md -->
<!-- Generated: 2026-05-02 | Updated: 2026-05-02 -->

# dashboard

## Purpose
Reserved namespace for dashboard-specific pages and components as the UI grows beyond the current Runs/Rules tabs (rule editor, workflow editor, settings). Currently empty in v0.1 — `App.tsx` lives one level up and shared components live under `../components/`.

## Subdirectories
| Directory | Purpose |
|-----------|---------|
| `components/` | (empty) — future home for dashboard-only components that don't belong in shared `../components/` |
| `pages/` | (empty) — future home for routed dashboard views |

## For AI Agents

### Working In This Directory
- Don't move existing `RulesPanel`/`RunsPanel` here without a reason — they're "shared" because the overlay window may eventually surface a tiny version.
- If you add `react-router-dom` routes, mount them from `App.tsx` and store route components here under `pages/`.

<!-- MANUAL: -->
