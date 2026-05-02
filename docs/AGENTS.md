<!-- Parent: ../AGENTS.md -->
<!-- Generated: 2026-05-02 | Updated: 2026-05-02 -->

# docs

## Purpose
Project documentation: architecture decision records, design notes for components that warrant their own write-up (e.g. workflow state machine), and image assets used by `README.md`/`SPEC.md`.

## Subdirectories
| Directory | Purpose |
|-----------|---------|
| `adr/` | Architecture Decision Records — numbered, never rewritten in place (see `adr/AGENTS.md`) |
| `design/` | In-depth design notes referenced from `SPEC.md` (see `design/AGENTS.md`) |
| `assets/` | PNG diagrams + hero images embedded in `README.md` |

## For AI Agents

### Working In This Directory
- ADRs follow the **immutable-once-merged** convention: never edit `001-tech-stack.md` to change a decision — add a new ADR that supersedes it.
- `SPEC.md` (in repo root) cross-references `docs/adr/001-tech-stack.md` and `docs/design/workflow-state-machine.md`. Keep those filenames stable; update `SPEC.md` if you rename them.
- Image assets in `assets/` are referenced from `README.md` via relative paths (`docs/assets/hero.png`). Don't move them without updating the README.

### Common Patterns
- Markdown only (no MDX), GitHub-flavored.
- Korean prose appears in `SPEC.md`; ADRs/design docs are English. Match the surrounding language in any file you edit.

<!-- MANUAL: -->
