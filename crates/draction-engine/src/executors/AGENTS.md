<!-- Parent: ../AGENTS.md -->
<!-- Generated: 2026-05-02 | Updated: 2026-05-02 -->

# executors

## Purpose
Implementations of the five built-in workflow node types. Each file defines a unit struct that implements `NodeExecutor` (`async fn execute(&self, params: &Value, ctx: &mut ExecCtx) -> anyhow::Result<()>`), and is registered in `default_registry()` keyed by its `node_type` string.

## Key Files
| File | Description |
|------|-------------|
| `mod.rs` | `pub mod` declarations for the five executors |
| `move_node.rs` | `MoveNode` (`type: "move"`) — moves the working file to `params.dest` (supports `~` expansion); updates `ctx.work_dir` |
| `copy_node.rs` | `CopyNode` (`type: "copy"`) — copies to `params.dest`, leaves the original in place |
| `rename_node.rs` | `RenameNode` (`type: "rename"`) — renames the working file based on `params.template` (supports `{name}`, `{ext}`, `{date}` substitutions) |
| `transcode_node.rs` | `TranscodeNode` (`type: "transcode"`) — shells out to bundled ffmpeg with `params.preset` (e.g. `h265_1080p`); writes alongside source and updates `ctx.work_dir` |
| `webhook_node.rs` | `WebhookNode` (`type: "webhook"`) — POSTs `params.url` with a JSON body referencing the run/event |

## For AI Agents

### Working In This Directory
- The `node_type` string is the registry key — must match the `WorkflowNode.node_type` JSON field. Don't change it without a migration plan for existing `~/Draction/workflows.json` files.
- ffmpeg in `transcode_node` is resolved via Tauri's resource path (bundled under `apps/draction-desktop/src-tauri/resources/bin/<platform>/`). The native shell falls back to `which ffmpeg` on `$PATH`.
- Webhook node uses `reqwest` with a 10s default timeout. Don't extend it — that's the SPEC's effective limit before users assume the workflow is hung.
- Move/rename should use `draction-inbox::file_ops::safe_move` to handle name collisions consistently with the ingest path.

### Testing Requirements
- `cargo test -p draction-engine` — each executor has tempdir-backed tests.

### Common Patterns
- All executors are zero-state unit structs.
- Errors carry context via `.with_context(|| format!(...))` so failure logs identify the node id.

## Dependencies

### Internal
- `draction-inbox` for file-ops helpers used by move/copy/rename

### External
- `reqwest` (webhook), bundled `ffmpeg` binary (transcode)

<!-- MANUAL: -->
