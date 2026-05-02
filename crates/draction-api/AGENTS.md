<!-- Parent: ../AGENTS.md -->
<!-- Generated: 2026-05-02 | Updated: 2026-05-02 -->

# draction-api

## Purpose
The local HTTP + WebSocket surface defined in `SPEC.md` §3. axum 0.8 server bound to `127.0.0.1` with port-walking from 9400..9410, bearer-token auth, and a `/api/v1/*` REST surface for rules/workflows/runs/events plus a `/ws` upgrade for live event streams. Used by external clients (planned OpenClaw bridge) and read paths from the Tauri/native frontends.

## Key Files
| File | Description |
|------|-------------|
| `Cargo.toml` | Depends on `axum` (with `ws`), `tower-http`, `tokio`, `serde`, `tracing`, plus all sibling Draction crates |
| `src/lib.rs` | `start_server(port, app_state)` — port-walks 10 ports, mounts router, returns the bound port |
| `src/state.rs` | `AppState { db, base_dir, auth_token, event_bus }` — shared across all handlers |
| `src/router.rs` | `build_router(state)` — wires handlers + middleware |
| `src/auth.rs` | `load_or_create_token(base_dir)` — generates a random bearer token at first run, persists to `~/Draction/config.json` |
| `src/ws.rs` | WebSocket upgrade handler — subscribes to the `EventBus` and forwards `Envelope`s as JSON frames |

## Subdirectories
| Directory | Purpose |
|-----------|---------|
| `src/handlers/` | One file per resource: `rules.rs`, `workflows.rs`, `runs.rs`, `events.rs` (see `src/handlers/AGENTS.md`) |
| `src/middleware/` | `auth.rs` (bearer token gate), `localhost.rs` (rejects non-127.0.0.1 connections) (see `src/middleware/AGENTS.md`) |

## For AI Agents

### Working In This Directory
- The server is **localhost-only by binding** (`SocketAddr::from(([127, 0, 0, 1], port))`) AND by middleware. Do not weaken either gate — the auth model assumes nobody can reach this port from the network.
- Port-walking starts at 9400 and tries up to +9 ports; the actual port is returned to callers and re-used by the Tauri `get_api_port` command.
- The token in `~/Draction/config.json` is persisted in plaintext at v0.1 — acceptable per `SPEC.md` §3 because the file lives in the user's home directory.
- All routes are mounted under `/api/v1`. The TS `packages/api-client` currently uses `/api/*` (no `/v1`) — that mismatch will need resolving before the bridge ships; treat axum as authoritative.

### Testing Requirements
- `cargo test -p draction-api` — handler-level tests using `axum::body` and tower's `oneshot`.
- Manual: `curl -H "Authorization: Bearer <token>" http://127.0.0.1:9400/api/v1/runs`.

### Common Patterns
- Handlers take `State<AppState>` + `Json<Body>` and return `Result<Json<T>, (StatusCode, Json<ErrorPayload>)>`.
- Error responses use `{ "error": { "code": "...", "message": "..." } }` per `SPEC.md` §3 "공통 에러 응답".
- WS messages are `{ "channel": "events", "payload": ... }` — defined in `draction-events::Envelope`.

## Dependencies

### Internal
- `draction-domain`, `draction-db`, `draction-events`

### External
- `axum` 0.8 (with `ws`), `tower-http` 0.6 (cors, trace), `tokio`, `serde`, `tracing`, `anyhow`

<!-- MANUAL: -->
