use crate::node_registry::{Artifact, NodeContext, NodeExecutor, NodeOutput};
use anyhow::Result;
use async_trait::async_trait;
use serde_json::Value;
use std::path::Path;
use tokio::process::Command;

pub struct TranscodeNode;

fn ffmpeg_args_for_preset(preset: &str) -> Result<Vec<&'static str>> {
    match preset {
        "h265_1080p" => Ok(vec![
            "-c:v", "libx265",
            "-crf", "28",
            "-preset", "medium",
            "-vf", "scale=-2:1080",
            "-c:a", "aac",
            "-b:a", "128k",
        ]),
        other => anyhow::bail!("transcode node: unknown preset '{}'", other),
    }
}

#[async_trait]
impl NodeExecutor for TranscodeNode {
    fn kind(&self) -> &'static str { "transcode" }

    async fn execute(&self, ctx: &NodeContext, params: Value) -> Result<NodeOutput> {
        // Check ffmpeg is available
        let which = Command::new("which")
            .arg("ffmpeg")
            .output()
            .await?;
        if !which.status.success() {
            anyhow::bail!("transcode node: ffmpeg is not installed or not in PATH");
        }

        let preset = params["preset"].as_str().unwrap_or("h265_1080p");
        let src = Path::new(&ctx.work_dir);

        if !src.exists() {
            return Err(anyhow::anyhow!("source file not found: {}", ctx.work_dir));
        }

        // Determine output path: same path with .mp4, or _transcoded.mp4 if already .mp4
        let output_path = {
            let ext = src.extension().and_then(|e| e.to_str()).unwrap_or("");
            if ext.eq_ignore_ascii_case("mp4") {
                let stem = src.file_stem().and_then(|s| s.to_str()).unwrap_or("output");
                let parent = src.parent().unwrap_or_else(|| Path::new("."));
                parent.join(format!("{}_transcoded.mp4", stem))
            } else {
                src.with_extension("mp4")
            }
        };

        let output_str = output_path.to_string_lossy().into_owned();
        let codec_args = ffmpeg_args_for_preset(preset)?;

        tracing::info!(
            preset,
            input = %ctx.work_dir,
            output = %output_str,
            "transcode node executing"
        );

        let mut cmd = Command::new("ffmpeg");
        cmd.arg("-i").arg(&ctx.work_dir);
        for arg in codec_args {
            cmd.arg(arg);
        }
        cmd.arg("-y").arg(&output_str);

        let status = cmd.status().await?;

        if !status.success() {
            anyhow::bail!("ffmpeg exited with code {:?}", status.code());
        }

        tracing::info!(output = %output_str, "transcode complete");

        Ok(NodeOutput {
            artifacts: vec![Artifact {
                kind: "file".into(),
                path: Some(output_str),
                url: None,
            }],
        })
    }
}
