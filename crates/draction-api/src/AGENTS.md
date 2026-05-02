<!-- Parent: ../AGENTS.md -->
<!-- Generated: 2026-05-02 | Updated: 2026-05-02 -->

# src

## Purpose
Internals of the `draction-api` crate. `lib.rs` exports `start_server`, the rest of this directory implements the routes, middleware, auth, shared state, and WebSocket plumbing.

## Key Files
| File | Description |
|------|-------------|
| `state.rs` | `AppState { db, base_dir, auth_token, event_bus }` — cloneable handle threaded into every handler |
| `auth.rs` | `load_or_create_token(base_dir)` — reads `~/Draction/config.json` or generates a fresh UUID-based token |
| `router.rs` | `build_router(state)` — composes handlers + middleware into a single `Router` |
| `ws.rs` | WS upgrade handler that subscribes to `EventBus` and forwards `Envelope`s as text frames |

## Subdirectories
| Directory | Purpose |
|-----------|---------|
| `handlers/` | Per-resource HTTP handlers (rules, workflows, runs, events) (see `handlers/AGENTS.md`) |
| `middleware/` | Bearer-auth + localhost-only gates (see `middleware/AGENTS.md`) |

## For AI Agents

### Working In This Directory
- The token in `auth.rs` is generated as a 256-bit hex string and stored as `{ "auth_token": "..." }` in `~/Draction/config.json`. Do not log this value.
- WS clients receive everything emitted on the bus from the moment they subscribe — there's no replay; clients catching up should call `GET /api/v1/runs` first.
- `AppState.event_bus` is `Arc<EventBus>`. Cloning it is `Arc::clone` — cheap.

### Common Patterns
- Handlers receive `State<AppState>` first, then path/body extractors.
- Errors return a typed `(StatusCode, Json<ErrorPayload>)` tuple — see `handlers/` for the consistent shape.

<!-- MANUAL: -->
