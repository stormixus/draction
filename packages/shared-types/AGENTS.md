<!-- Parent: ../AGENTS.md -->
<!-- Generated: 2026-05-02 | Updated: 2026-05-02 -->

# shared-types

## Purpose
Pure TypeScript domain types intended to mirror `crates/draction-domain`. Consumed by `packages/api-client` and (eventually) the OpenClaw bridge.

## Key Files
| File | Description |
|------|-------------|
| `package.json` | `@draction/shared-types`, ESM, no build step (`main`/`types` point at `src/index.ts`) |
| `src/index.ts` | All type exports: `Rule`, `Condition`, `Op`, `ThenAction`, `Workflow`, `WorkflowNode`, `Edge`, `Run`, `RunStatus`, `NodeStatus`, `IngestEvent`, `IngestFile`, `AppState` |
| `tsconfig.json` | Strict TS config used by the `typecheck` script |

## For AI Agents

### Working In This Directory
- These types **drift from the Rust source of truth** in `crates/draction-domain`. Known divergence:
  - `Rule` here uses `priority` / `condition` / `then_action` / `created_at` / `updated_at`; Rust uses `order_index` / `when` / `then` / no timestamps.
  - `Condition` is tagged-union here (`{ type: "group" | "predicate" }`); Rust uses serde's untagged variants in the JSON.
  - `Op` here lacks `nin`/`contains` etc. — match the Rust `Op` enum.
- When in doubt, treat Rust as authoritative; update this file rather than introducing a parallel-but-different shape.
- This package has **no runtime code** and ships its `.ts` directly. Don't add a build step that emits `.d.ts` unless a consumer needs it.

### Testing Requirements
- `pnpm -F @draction/shared-types typecheck` — runs `tsc --noEmit`.

### Common Patterns
- Interfaces over `type` aliases for object shapes; `type` for unions and primitives.
- ISO-8601 strings for all timestamps (matches the API's RFC3339 output).

## Dependencies

### External
- TypeScript 5.7 (devDependency only)

<!-- MANUAL: -->
