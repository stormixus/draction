<!-- Parent: ../AGENTS.md -->
<!-- Generated: 2026-05-02 | Updated: 2026-05-02 -->

# scripts

## Purpose
One-off Python utilities for processing the Draky character sprite assets shipped under `apps/*/public/sprites/`. Not part of any runtime; run manually when regenerating sprite sheets.

## Key Files
| File | Description |
|------|-------------|
| `analyze.py` | Inspect a sprite sheet's geometry / channel histograms |
| `flood_transparent.py` | Flood-fill background pixels to alpha=0 for clean edges |
| `remove_white.py` | Strip near-white backgrounds, replacing with transparency |

## For AI Agents

### Working In This Directory
- These scripts are **author tools**, not part of the build pipeline. CI does not invoke them.
- They run against the project's `.venv/` (top-level). Activate it (or use `uv`/`pip`) before invoking — there is no requirements file checked in.
- Outputs typically overwrite `apps/draction-desktop/public/sprites/draky-sheet.png` and the matching landing copy. Confirm both paths after regenerating.

### Common Patterns
- Pillow (PIL) is the only third-party dependency.
- Stand-alone CLI scripts — no shared module structure.

## Dependencies

### External
- Python 3 + Pillow (installed in `.venv/`)

<!-- MANUAL: -->
