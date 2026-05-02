use crate::node_registry::{Artifact, NodeContext, NodeExecutor, NodeOutput};
use anyhow::Result;
use async_trait::async_trait;
use serde_json::Value;
use std::path::Path;

pub struct CopyNode;

#[async_trait]
impl NodeExecutor for CopyNode {
    fn kind(&self) -> &'static str { "copy" }

    async fn execute(&self, ctx: &NodeContext, params: Value) -> Result<NodeOutput> {
        let dest_dir = params["dest"].as_str()
            .ok_or_else(|| anyhow::anyhow!("copy node: missing 'dest' param"))?;

        // Expand ~ to home dir
        let dest_dir = if let Some(rest) = dest_dir.strip_prefix("~/") {
            dirs::home_dir()
                .ok_or_else(|| anyhow::anyhow!("cannot resolve home dir"))?
                .join(rest)
        } else {
            std::path::PathBuf::from(dest_dir)
        };

        let src = Path::new(&ctx.work_dir);
        if !src.exists() {
            return Err(anyhow::anyhow!("source file not found: {}", ctx.work_dir));
        }

        tokio::fs::create_dir_all(&dest_dir).await?;

        let file_name = src.file_name().unwrap_or_default();
        let dest_file = dest_dir.join(file_name);

        tokio::fs::copy(src, &dest_file).await?;

        tracing::info!(src = %ctx.work_dir, dest = %dest_file.display(), "file copied");

        Ok(NodeOutput {
            artifacts: vec![Artifact {
                kind: "file".into(),
                path: Some(dest_file.to_string_lossy().into()),
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
    fn kind_returns_copy() {
        assert_eq!(CopyNode.kind(), "copy");
    }

    #[tokio::test]
    async fn copies_file_and_keeps_original() {
        let src_dir = tempdir().unwrap();
        let dest_dir = tempdir().unwrap();
        let src_path = src_dir.path().join("note.md");
        tokio::fs::write(&src_path, b"keep me").await.unwrap();

        let params = json!({ "dest": dest_dir.path().to_string_lossy() });
        let out = CopyNode.execute(&ctx(&src_path), params).await.unwrap();

        let copied = dest_dir.path().join("note.md");
        assert!(src_path.exists(), "copy must leave the original in place");
        assert!(copied.exists(), "copy must create the destination file");
        assert_eq!(tokio::fs::read(&copied).await.unwrap(), b"keep me");
        assert_eq!(out.artifacts[0].path.as_deref().unwrap(), copied.to_string_lossy());
    }

    #[tokio::test]
    async fn creates_dest_directory_when_missing() {
        let src_dir = tempdir().unwrap();
        let dest_root = tempdir().unwrap();
        let nested = dest_root.path().join("a").join("b");
        let src_path = src_dir.path().join("file.txt");
        tokio::fs::write(&src_path, b"hi").await.unwrap();

        let params = json!({ "dest": nested.to_string_lossy() });
        CopyNode.execute(&ctx(&src_path), params).await.unwrap();
        assert!(nested.join("file.txt").exists());
    }

    #[tokio::test]
    async fn missing_dest_param_returns_error() {
        let dir = tempdir().unwrap();
        let src = dir.path().join("a.txt");
        tokio::fs::write(&src, b"x").await.unwrap();
        let err = CopyNode.execute(&ctx(&src), json!({})).await.unwrap_err();
        assert!(err.to_string().contains("'dest'"));
    }

    #[tokio::test]
    async fn missing_source_returns_error() {
        let dir = tempdir().unwrap();
        let dest = tempdir().unwrap();
        let phantom = dir.path().join("nope.bin");
        let params = json!({ "dest": dest.path().to_string_lossy() });
        let err = CopyNode.execute(&ctx(&phantom), params).await.unwrap_err();
        assert!(err.to_string().contains("source file not found"));
    }
}
