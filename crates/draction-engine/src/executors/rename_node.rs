use crate::node_registry::{Artifact, NodeContext, NodeExecutor, NodeOutput};
use anyhow::Result;
use async_trait::async_trait;
use chrono::Local;
use serde_json::Value;
use std::path::Path;

pub struct RenameNode;

#[async_trait]
impl NodeExecutor for RenameNode {
    fn kind(&self) -> &'static str { "rename" }

    async fn execute(&self, ctx: &NodeContext, params: Value) -> Result<NodeOutput> {
        let pattern = params["pattern"].as_str().unwrap_or("{name}.{ext}");

        let src = Path::new(&ctx.work_dir);
        if !src.exists() {
            return Err(anyhow::anyhow!("source file not found: {}", ctx.work_dir));
        }

        let stem = src.file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("");
        let ext = src.extension()
            .and_then(|s| s.to_str())
            .unwrap_or("");
        let date = Local::now().format("%Y-%m-%d").to_string();

        let new_name = pattern
            .replace("{name}", stem)
            .replace("{ext}", ext)
            .replace("{date}", &date);

        let parent = src.parent().unwrap_or_else(|| Path::new("."));
        let dest = parent.join(&new_name);

        tokio::fs::rename(src, &dest).await?;

        tracing::info!(src = %ctx.work_dir, dest = %dest.display(), "file renamed");

        Ok(NodeOutput {
            artifacts: vec![Artifact {
                kind: "file".into(),
                path: Some(dest.to_string_lossy().into()),
                url: None,
            }],
        })
    }
}
