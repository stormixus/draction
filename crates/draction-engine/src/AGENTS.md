<!-- Parent: ../AGENTS.md -->
<!-- Generated: 2026-05-02 | Updated: 2026-05-02 -->

# src

## Purpose
Internals of the engine crate. `lib.rs` builds the default executor registry; the rule and workflow engines live alongside, and the per-node implementations live under `executors/`.

## Key Files
| File | Description |
|------|-------------|
| `rule_engine.rs` | `match_first_rule(rules, ctx)` + `EvalCtx` (`HashMap<String, serde_json::Value>`) + `Op` evaluator |
| `workflow_engine.rs` | `WorkflowEngine` — topological execution of `Workflow.nodes` against the registry |
| `node_registry.rs` | `NodeRegistry { handlers: HashMap<String, Box<dyn NodeExecutor>> }` + `register()` + `get()` |

## Subdirectories
| Directory | Purpose |
|-----------|---------|
| `executors/` | Per-node-type implementations (see `executors/AGENTS.md`) |

## For AI Agents

### Working In This Directory
- The rule engine's `EvalCtx` keys are *flat* — there's no dot-path resolution (`file.ext` is **not** supported; the caller must populate `ext` directly). Match the keys produced by `app-core::build_eval_ctx`: `name`, `ext`, `size_bytes`.
- Workflow execution is **fail-fast and serial** at v0.1. Failures bubble up to `app-core` which records `RUN_FAILED`.
- Adding a new `Op` variant: extend `Op` in `draction-domain::rule`, then add the match arm in `rule_engine::evaluate_predicate`. Don't forget the JSON serde rename.

### Testing Requirements
- `cargo test -p draction-engine` — the rule engine has table-driven tests; new `Op`s should add cases.

### Common Patterns
- Pure-functional rule eval — no shared state, no I/O.
- Workflow engine emits `tracing` spans per node so failures are diagnosable in the API server's stdout.

<!-- MANUAL: -->
