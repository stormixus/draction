<!-- Parent: ../AGENTS.md -->
<!-- Generated: 2026-05-02 | Updated: 2026-05-02 -->

# draction-events

## Purpose
In-process pub/sub for event payloads. Wraps `tokio::sync::broadcast` so the runtime can fan events out to multiple subscribers (currently: the WebSocket handler in `draction-api`, future: an OpenClaw bridge). Defines the `Envelope { channel, payload }` wire format that maps directly to the WS message format in `SPEC.md` §3.

## Key Files
| File | Description |
|------|-------------|
| `Cargo.toml` | Depends on `tokio` (broadcast feature), `serde`, `serde_json` |
| `src/lib.rs` | Re-exports `EventBus`, `Envelope` |
| `src/bus.rs` | `EventBus::new(capacity)` + `emit()` + `subscribe()` over `tokio::sync::broadcast::Sender<Envelope>` |
| `src/envelope.rs` | `Envelope { channel: String, payload: serde_json::Value }` — the WS frame body |

## For AI Agents

### Working In This Directory
- The bus is **best-effort and bounded** (default capacity 256 in `app-core`). Slow subscribers may lag and drop messages — design subscribers to tolerate `RecvError::Lagged`.
- Single global bus per process; clone the `Arc<EventBus>` to share it. Don't construct multiple buses.
- `Envelope.channel` is currently always `"events"`. The field exists for future segmentation (e.g. `"diagnostics"`).

### Testing Requirements
- `cargo test -p draction-events` — broadcast round-trip tests.

### Common Patterns
- Payloads are `serde_json::Value` (not strongly typed) so emitters can shape arbitrary `EVENT_INGESTED` / `RUN_*` JSON without a central enum.

## Dependencies

### External
- `tokio` (broadcast), `serde`, `serde_json`

<!-- MANUAL: -->
