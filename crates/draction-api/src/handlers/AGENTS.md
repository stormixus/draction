<!-- Parent: ../AGENTS.md -->
<!-- Generated: 2026-05-02 | Updated: 2026-05-02 -->

# handlers

## Purpose
One file per top-level resource in the v0.1 API. Each file exports the axum handler functions and a `routes()` function returning a `Router` segment that `router::build_router` merges in.

## Key Files
| File | Description |
|------|-------------|
| `mod.rs` | Re-exports the four submodules |
| `rules.rs` | `GET /rules`, `GET /rules/:id`, `POST /rules`, `PUT /rules/:id`, `DELETE /rules/:id`, `PATCH /rules/:id/enabled` — backed by `~/Draction/rules.json` |
| `workflows.rs` | `GET /workflows`, `GET /workflows/:id`, `POST /workflows`, `PUT /workflows/:id` — backed by `~/Draction/workflows.json` |
| `runs.rs` | `GET /runs?status=&limit=`, `GET /runs/:id`, `POST /runs/:id/retry` — pulls from SQLite |
| `events.rs` | `GET /events`, `POST /events/:eventId/undo` — undo returns 409 if a run for the event is in flight |

## For AI Agents

### Working In This Directory
- Rules and workflows live in JSON files, NOT SQLite (per `app-core::ensure_rules`/`ensure_workflows`). Mutations rewrite the whole file under a lock; concurrent edits via the API will race with manual edits to the file.
- Status-code conventions per `SPEC.md` §3: 400 (validation), 401 (auth), 404 (missing), 409 (conflict), 500 (internal). Use these consistently — clients depend on them.
- The `retry` endpoint creates a new run; it doesn't mutate the original.

### Testing Requirements
- Handler-level tests using `tower::ServiceExt::oneshot` against a `Router` built with an in-memory `AppState`.

### Common Patterns
- Handlers return `Result<Json<T>, ApiError>` where `ApiError` produces the `{ "error": { "code", "message" } }` body in `IntoResponse`.

<!-- MANUAL: -->
