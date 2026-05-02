<!-- Parent: ../AGENTS.md -->
<!-- Generated: 2026-05-02 | Updated: 2026-05-02 -->

# design

## Purpose
In-depth design notes for components large enough to warrant their own document. Referenced from `SPEC.md` (e.g. §6 points to the workflow state machine design here).

## Key Files
| File | Description |
|------|-------------|
| `workflow-state-machine.md` | Per-node state transitions (queued/running/success/failed/skipped), retry semantics, fail-fast policy. Referenced from `SPEC.md` §6 |

## For AI Agents

### Working In This Directory
- Unlike ADRs, these documents **are** living — update them when the design they describe changes. Note the change in a "Revisions" section at the bottom rather than rewriting silently.
- Cross-references from `SPEC.md` and code comments use relative paths (`docs/design/workflow-state-machine.md`); keep filenames stable.

### Common Patterns
- Mermaid or ASCII diagrams are fine. Avoid binary diagram formats — they don't review well in PRs.

<!-- MANUAL: -->
