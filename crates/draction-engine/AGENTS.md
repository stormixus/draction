<!-- Parent: ../AGENTS.md -->
<!-- Generated: 2026-05-02 | Updated: 2026-05-02 -->

# draction-engine

## Purpose
The execution core. Two engines live here:
1. **Rule engine** — evaluates a Hazel-style condition tree (`Group(All|Any)` of `Predicate{field,op,value}`) against an `EvalCtx` map and returns the first matching rule (first-match-wins per `SPEC.md` §5).
2. **Workflow engine** — runs an n8n-lite DAG of `WorkflowNode`s in topological order using a registry of executors. v0.1 nodes: `move`, `copy`, `rename`, `transcode`, `webhook`.

## Key Files
| File | Description |
|------|-------------|
| `Cargo.toml` | Depends on `draction-domain`, `draction-inbox`, `tokio`, `serde`, `serde_json`, `async-trait`, `anyhow`, `tracing` |
| `src/lib.rs` | Module declarations + `default_registry()` constructor that wires all five built-in node types |
| `src/rule_engine.rs` | `match_first_rule(rules, ctx)` + `EvalCtx` (HashMap<String, Value>) |
| `src/workflow_engine.rs` | `WorkflowEngine::new(registry)` + `execute(run_id, event_id, workflow, work_dir)` |
| `src/node_registry.rs` | `NodeRegistry` keyed by `node_type` string → `Box<dyn NodeExecutor>` trait object |

## Subdirectories
| Directory | Purpose |
|-----------|---------|
| `src/executors/` | One file per built-in node type (move/copy/rename/transcode/webhook) (see `src/executors/AGENTS.md`) |

## For AI Agents

### Working In This Directory
- v0.1 is **fail-fast and serial**: a node failure aborts the workflow with no automatic retry, and only one workflow runs at a time per process. v0.2 is slated to add per-node retry + parallelism.
- New node types: implement `NodeExecutor`, add a `mod` line in `executors/mod.rs`, register it in `default_registry()` at the top of `lib.rs`. The registry's key is the `node_type` string in `WorkflowNode`.
- The rule engine is **purely functional** — it doesn't read or write the DB. Callers compute `EvalCtx` (typically `name`/`ext`/`size_bytes`) and pass it in.
- `Op::In` checks JSON-array membership; mismatched types return `false` rather than erroring.

### Testing Requirements
- `cargo test -p draction-engine` — table-driven tests for each operator and a workflow execution smoke test against tempdirs.

### Common Patterns
- Executors are `#[async_trait]` `NodeExecutor`s returning `anyhow::Result<()>`.
- The workflow's `work_dir` is the **current path of the in-flight file** (after the move into Inbox); executors use it as both input and the running cursor.
- `tracing::info!`/`error!` for execution telemetry — these surface in the API server's stdout.

## Dependencies

### Internal
- `draction-domain` (Rule/Workflow types)
- `draction-inbox` (file-ops helpers used by move/copy/rename executors)

### External
- `tokio`, `serde`, `serde_json`, `async-trait`, `anyhow`, `tracing`, plus per-executor deps (e.g. `reqwest` for webhook, ffmpeg shelled out by transcode)

<!-- MANUAL: -->
