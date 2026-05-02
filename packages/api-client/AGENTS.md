<!-- Parent: ../AGENTS.md -->
<!-- Generated: 2026-05-02 | Updated: 2026-05-02 -->

# api-client

## Purpose
TypeScript fetch wrapper for Draction's local HTTP API. Exposes a `DractionClient` class with typed `listRules`/`createRule`/`listWorkflows`/`listRuns`/`listEvents`/`undo`/`pair`/`health` methods. Default base URL `http://127.0.0.1:9400`.

## Key Files
| File | Description |
|------|-------------|
| `package.json` | `@draction/api-client`, ESM, depends on `@draction/shared-types: workspace:*` |
| `src/index.ts` | `DractionClient` class + re-exports of the shared types |
| `tsconfig.json` | Strict TS config used by the `typecheck` script |

## For AI Agents

### Working In This Directory
- This client uses **`/api/...`** paths but the axum server actually mounts under **`/api/v1/...`** (see `crates/draction-api/src/lib.rs`). That mismatch will break real requests — fix when the bridge starts using this client. The Rust server is authoritative.
- Bearer-token auth is supported via `setToken(token)` or returned from `pair()`. The pair endpoint per `SPEC.md` §3 requires user approval in the Draction UI; this client just performs the HTTP call.
- The class is **stateless except for the token**. Construct one per app or memoize a singleton — both are fine.

### Testing Requirements
- `pnpm -F @draction/api-client typecheck`.
- No runtime tests at v0.1.

### Common Patterns
- All requests go through the private `request<T>(method, path, body?)` helper.
- Errors throw `Error` with the response body in the message — callers should `try`/`catch` and surface user-facing failure UI.
- Returns are `Promise<T>` of the shared-types shape.

## Dependencies

### Internal
- `@draction/shared-types`

### External
- TypeScript 5.7 (devDependency only)

<!-- MANUAL: -->
