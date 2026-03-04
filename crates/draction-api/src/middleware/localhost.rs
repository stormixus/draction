use axum::{extract::{ConnectInfo, Request}, middleware::Next, response::Response};
use axum::http::StatusCode;
use std::net::SocketAddr;

pub async fn localhost_only(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    if !addr.ip().is_loopback() {
        return Err(StatusCode::FORBIDDEN);
    }
    Ok(next.run(req).await)
}
