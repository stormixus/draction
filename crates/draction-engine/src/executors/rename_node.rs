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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use tempfile::tempdir;

    fn ctx(work_dir: &std::path::Path) -> NodeContext {
        NodeContext {
            run_id: "r".into(),
            event_id: "e".into(),
            work_dir: work_dir.to_string_lossy().into_owned(),
        }
    }

    #[test]
    fn kind_returns_rename() {
        assert_eq!(RenameNode.kind(), "rename");
    }

    #[tokio::test]
    async fn default_pattern_keeps_original_name() {
        let dir = tempdir().unwrap();
        let src = dir.path().join("doc.txt");
        tokio::fs::write(&src, b"x").await.unwrap();

        RenameNode.execute(&ctx(&src), json!({})).await.unwrap();
        // Default pattern is "{name}.{ext}" → same effective name
        assert!(dir.path().join("doc.txt").exists());
    }

    #[tokio::test]
    async fn pattern_substitutes_name_and_ext_tokens() {
        let dir = tempdir().unwrap();
        let src = dir.path().join("photo.JPG");
        tokio::fs::write(&src, b"x").await.unwrap();

        let params = json!({ "pattern": "renamed_{name}.{ext}" });
        let out = RenameNode.execute(&ctx(&src), params).await.unwrap();

        let expected = dir.path().join("renamed_photo.JPG");
        assert!(expected.exists(), "renamed file should exist");
        assert!(!src.exists(), "original name should no longer exist");
        assert_eq!(out.artifacts[0].path.as_deref().unwrap(), expected.to_string_lossy());
    }

    #[tokio::test]
    async fn pattern_substitutes_date_token() {
        let dir = tempdir().unwrap();
        let src = dir.path().join("scan.pdf");
        tokio::fs::write(&src, b"x").await.unwrap();

        let params = json!({ "pattern": "{date}_{name}.{ext}" });
        RenameNode.execute(&ctx(&src), params).await.unwrap();

        // The file must exist with a YYYY-MM-DD prefix; we don't pin the date.
        let mut entries = tokio::fs::read_dir(dir.path()).await.unwrap();
        let mut found_date_prefixed = false;
        while let Some(entry) = entries.next_entry().await.unwrap() {
            let name = entry.file_name().to_string_lossy().into_owned();
            if name.ends_with("_scan.pdf") && name.len() == "YYYY-MM-DD_scan.pdf".len() {
                found_date_prefixed = true;
                break;
            }
        }
        assert!(found_date_prefixed, "expected a YYYY-MM-DD_scan.pdf file");
    }

    #[tokio::test]
    async fn missing_source_returns_error() {
        let dir = tempdir().unwrap();
        let phantom = dir.path().join("missing.txt");
        let err = RenameNode.execute(&ctx(&phantom), json!({})).await.unwrap_err();
        assert!(err.to_string().contains("source file not found"));
    }
}
