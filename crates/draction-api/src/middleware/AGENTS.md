<!-- Parent: ../AGENTS.md -->
<!-- Generated: 2026-05-02 | Updated: 2026-05-02 -->

# middleware

## Purpose
axum middleware applied to all routes under `/api/v1`. Implements the SPEC.md §3 security model: localhost-only AND bearer-token auth, applied in that order.

## Key Files
| File | Description |
|------|-------------|
| `mod.rs` | Re-exports the two middleware modules |
| `localhost.rs` | Rejects requests whose remote socket address is not `127.0.0.1`/`::1` — defense-in-depth on top of the bind address |
| `auth.rs` | Validates the `Authorization: Bearer <token>` header against `AppState.auth_token`; returns 401 on mismatch |

## For AI Agents

### Working In This Directory
- These two layers are **redundant by design**: bind + middleware. Removing either weakens the security posture beyond what `SPEC.md` permits.
- The auth middleware **must not** log the token or include it in error messages.
- The `/api/v1/health` endpoint sits outside this directory's middleware on purpose — it's reachable for liveness checks. Don't move it under the gated router.

### Common Patterns
- Tower-style `axum::middleware::from_fn` adapters that return `Result<Response, StatusCode>`.

<!-- MANUAL: -->
