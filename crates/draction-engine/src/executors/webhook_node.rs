use crate::node_registry::{Artifact, NodeContext, NodeExecutor, NodeOutput};
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use serde_json::{json, Value};

pub struct WebhookNode;

#[async_trait]
impl NodeExecutor for WebhookNode {
    fn kind(&self) -> &'static str {
        "webhook"
    }

    async fn execute(&self, ctx: &NodeContext, params: Value) -> Result<NodeOutput> {
        let url = params["url"]
            .as_str()
            .ok_or_else(|| anyhow!("webhook node requires 'url' param"))?;

        tracing::info!(url, "webhook node executing");

        let body = json!({
            "run_id": ctx.run_id,
            "event_id": ctx.event_id,
            "file": ctx.work_dir,
        });

        let client = reqwest::Client::new();
        let response = client
            .post(url)
            .json(&body)
            .send()
            .await
            .map_err(|e| anyhow!("webhook POST failed: {e}"))?;

        if !response.status().is_success() {
            return Err(anyhow!(
                "webhook returned non-2xx status: {}",
                response.status()
            ));
        }

        Ok(NodeOutput {
            artifacts: vec![Artifact {
                kind: "link".into(),
                path: None,
                url: Some(url.to_string()),
            }],
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::net::TcpListener;

    fn ctx(work_dir: &str) -> NodeContext {
        NodeContext {
            run_id: "run-x".into(),
            event_id: "evt-y".into(),
            work_dir: work_dir.into(),
        }
    }

    #[test]
    fn kind_returns_webhook() {
        assert_eq!(WebhookNode.kind(), "webhook");
    }

    #[tokio::test]
    async fn missing_url_param_errors_without_network_call() {
        let err = WebhookNode.execute(&ctx("/tmp/x"), json!({})).await.unwrap_err();
        assert!(err.to_string().contains("'url'"));
    }

    /// Tiny single-shot HTTP responder. Reads the request bytes (so the client gets a clean
    /// write side), then returns those bytes plus the chosen status to the test.
    async fn one_shot_server(status_line: &'static str) -> (String, tokio::task::JoinHandle<Vec<u8>>) {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let url = format!("http://{}/hook", addr);
        let handle = tokio::spawn(async move {
            let (mut socket, _) = listener.accept().await.unwrap();
            let mut buf = vec![0u8; 8192];
            let mut total = Vec::new();
            // Best-effort read until we see the end of headers; for tests bodies are small.
            loop {
                let n = socket.read(&mut buf).await.unwrap_or(0);
                if n == 0 { break; }
                total.extend_from_slice(&buf[..n]);
                if total.windows(4).any(|w| w == b"\r\n\r\n") {
                    break;
                }
            }
            let response = format!(
                "HTTP/1.1 {}\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
                status_line
            );
            let _ = socket.write_all(response.as_bytes()).await;
            let _ = socket.shutdown().await;
            total
        });
        (url, handle)
    }

    #[tokio::test]
    async fn happy_path_posts_run_and_event_metadata() {
        let (url, server) = one_shot_server("200 OK").await;
        let params = json!({ "url": url });
        let out = WebhookNode.execute(&ctx("/tmp/file.txt"), params).await.unwrap();

        let request = server.await.unwrap();
        let req_str = String::from_utf8_lossy(&request);

        assert!(req_str.starts_with("POST "), "expected a POST request");
        assert!(req_str.contains("\"run_id\":\"run-x\""));
        assert!(req_str.contains("\"event_id\":\"evt-y\""));
        assert!(req_str.contains("\"file\":\"/tmp/file.txt\""));

        // Artifact reflects the URL as a link
        assert_eq!(out.artifacts[0].kind, "link");
        assert!(out.artifacts[0].url.is_some());
    }

    #[tokio::test]
    async fn non_2xx_response_returns_error() {
        let (url, server) = one_shot_server("500 Internal Server Error").await;
        let params = json!({ "url": url });
        let err = WebhookNode.execute(&ctx("/tmp/x"), params).await.unwrap_err();
        let _ = server.await;
        assert!(err.to_string().contains("non-2xx"));
        assert!(err.to_string().contains("500"));
    }

    #[tokio::test]
    async fn unreachable_host_surfaces_post_failure() {
        // 127.0.0.1:1 is reserved/unused — connect must fail fast
        let params = json!({ "url": "http://127.0.0.1:1/no" });
        let err = WebhookNode.execute(&ctx("/tmp/x"), params).await.unwrap_err();
        assert!(err.to_string().contains("webhook POST failed"));
    }
}
