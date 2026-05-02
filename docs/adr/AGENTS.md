<!-- Parent: ../AGENTS.md -->
<!-- Generated: 2026-05-02 | Updated: 2026-05-02 -->

# adr

## Purpose
Architecture Decision Records — numbered markdown files capturing **why** a structural choice was made (Tauri vs Electron, etc.). Each ADR is immutable once merged; revisions are new ADRs that supersede earlier ones.

## Key Files
| File | Description |
|------|-------------|
| `001-tech-stack.md` | The v0.1 tech-stack decision (Tauri + Rust workspace, axum, SQLite, React). Referenced from `SPEC.md` §9 |

## For AI Agents

### Working In This Directory
- **Never edit a merged ADR's decision content.** Add a new ADR (`002-*.md`, `003-*.md`, ...) that supersedes the old one and link both ways.
- File names follow `NNN-kebab-title.md` with zero-padded numbers.
- ADRs are referenced by relative path from `SPEC.md` and `README.md` — keep filenames stable.

### Common Patterns
- Standard ADR structure: Context → Decision → Consequences → Alternatives Considered.

<!-- MANUAL: -->
