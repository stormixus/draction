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
