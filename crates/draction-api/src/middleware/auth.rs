use axum::{
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

pub async fn bearer_auth(
    req: Request,
    next: Next,
    expected_token: String,
) -> Response {
    // Skip auth for health endpoint
    if req.uri().path() == "/health" && req.method() == axum::http::Method::GET {
        return next.run(req).await;
    }

    let auth_header = req
        .headers()
        .get("authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .map(|s| s.to_owned());

    match auth_header {
        Some(token) if token == expected_token => next.run(req).await,
        _ => (
            StatusCode::UNAUTHORIZED,
            Json(json!({
                "error": {
                    "code": "UNAUTHORIZED",
                    "message": "Missing or invalid Bearer token"
                }
            })),
        )
            .into_response(),
    }
}
