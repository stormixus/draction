<!-- Parent: ../AGENTS.md -->
<!-- Generated: 2026-05-02 | Updated: 2026-05-02 -->

# draction-domain

## Purpose
Pure data crate. Defines the canonical Rust shapes for events, rules, workflows, and ID generation. No I/O, no async, no dependencies on other Draction crates. This is the **source of truth** that TypeScript `packages/shared-types` mirrors.

## Key Files
| File | Description |
|------|-------------|
| `Cargo.toml` | Manifest — depends only on `serde`, `serde_json`, `chrono`, `uuid` from the workspace |
| `src/lib.rs` | Re-exports the four submodules (`event`, `rule`, `workflow`, `ids`) |
| `src/event.rs` | `EventIngested`, file/source payload types — matches `SPEC.md` §4 schema |
| `src/rule.rs` | `Rule`, `Condition` (Group/Predicate enum), `Op` (`Eq`/`In`/`Gt`/`Lt`/...), `ThenAction` |
| `src/workflow.rs` | `Workflow`, `WorkflowNode`, `Edge` |
| `src/ids.rs` | ID minting helpers: `new_event_id` → `evt_<uuid>`, `new_run_id` → `run_<uuid>`, etc. |

## For AI Agents

### Working In This Directory
- **Never add runtime behavior here** — keep the crate `no_std`-friendly in spirit (currently uses `std` for `String` but no I/O). Any function that touches the filesystem, network, or DB belongs in another crate.
- Field renames cascade: changing a serde field name in `Rule` will silently break existing `~/Draction/rules.json` files on user machines. Bump compatibility carefully.
- `Op::In` accepts a JSON array as its `value`; other ops accept scalars. The engine's `rule_engine` is the canonical interpreter.
- ID prefixes are the API contract — `evt_`, `run_`, `rule_`, `wf_`. Don't change them.

### Testing Requirements
- `cargo test -p draction-domain` — minimal, mostly serde round-trip tests if any.

### Common Patterns
- All types derive `Serialize, Deserialize, Debug, Clone`.
- `serde(rename_all = "camelCase")` is **not** universally applied — fields like `order_index` serialize snake-case while events use camelCase per the SPEC. Match the surrounding file's convention.
- Strings for IDs (newtypes were considered but not adopted at v0.1).

## Dependencies

### External
- `serde`, `serde_json`, `chrono`, `uuid`

<!-- MANUAL: -->
