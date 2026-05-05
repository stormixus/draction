use crate::node_registry::{Artifact, NodeContext, NodeExecutor, NodeOutput};
use anyhow::Result;
use async_trait::async_trait;
use serde_json::Value;
use std::path::Path;
use tokio::process::Command;

pub struct TranscodeNode;

fn ffmpeg_args_for_preset(preset: &str) -> Result<Vec<&'static str>> {
    match preset {
        "h264_720p" => Ok(vec![
            "-c:v", "libx264",
            "-preset", "fast",
            "-crf", "23",
            "-vf", "scale=-2:720",
            "-c:a", "aac",
            "-b:a", "128k",
        ]),
        "h265_1080p" => Ok(vec![
            "-c:v", "libx265",
            "-crf", "28",
            "-preset", "medium",
            "-vf", "scale=-2:1080",
            "-c:a", "aac",
            "-b:a", "128k",
        ]),
        "h265_4k" => Ok(vec![
            "-c:v", "libx265",
            "-preset", "medium",
            "-crf", "20",
            "-vf", "scale=-2:2160",
            "-c:a", "aac",
            "-b:a", "192k",
        ]),
        "audio_only" => Ok(vec![
            "-vn",
            "-c:a", "libmp3lame",
            "-b:a", "320k",
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

        // Determine output extension based on preset
        let out_ext = if preset == "audio_only" { "mp3" } else { "mp4" };

        // Determine output path: same path with new extension, or _transcoded.<ext> if already that extension
        let output_path = {
            let ext = src.extension().and_then(|e| e.to_str()).unwrap_or("");
            if ext.eq_ignore_ascii_case(out_ext) {
                let stem = src.file_stem().and_then(|s| s.to_str()).unwrap_or("output");
                let parent = src.parent().unwrap_or_else(|| Path::new("."));
                parent.join(format!("{}_transcoded.{}", stem, out_ext))
            } else {
                src.with_extension(out_ext)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kind_returns_transcode() {
        assert_eq!(TranscodeNode.kind(), "transcode");
    }

    #[test]
    fn h265_1080p_preset_returns_libx265_codec_chain() {
        let args = ffmpeg_args_for_preset("h265_1080p").unwrap();
        assert!(args.contains(&"libx265"), "preset must use libx265 codec");
        assert!(args.contains(&"scale=-2:1080"), "preset must scale to 1080p");
        assert!(args.contains(&"aac"), "preset must transcode audio to aac");
        // -crf with a value
        let crf_idx = args.iter().position(|a| *a == "-crf").expect("missing -crf flag");
        assert!(crf_idx + 1 < args.len(), "-crf must be followed by a value");
    }

    #[test]
    fn unknown_preset_errors_with_preset_name_in_message() {
        let err = ffmpeg_args_for_preset("h264_potato").unwrap_err();
        let msg = err.to_string();
        assert!(msg.contains("unknown preset"));
        assert!(msg.contains("h264_potato"));
    }

    #[test]
    fn ffmpeg_args_are_pairs_of_flag_and_value() {
        // Sanity: every -* flag should have a non-flag value following it
        let args = ffmpeg_args_for_preset("h265_1080p").unwrap();
        let mut i = 0;
        while i < args.len() {
            if args[i].starts_with('-') {
                assert!(i + 1 < args.len(), "flag {} has no value", args[i]);
                i += 2;
            } else {
                i += 1;
            }
        }
    }

    #[test]
    fn h264_720p_preset_returns_libx264_codec_chain() {
        let args = ffmpeg_args_for_preset("h264_720p").unwrap();
        assert!(args.contains(&"libx264"), "preset must use libx264 codec");
        assert!(args.contains(&"scale=-2:720"), "preset must scale to 720p");
        assert!(args.contains(&"aac"), "preset must transcode audio to aac");
        assert!(args.contains(&"fast"), "preset must use fast encoding preset");
        let crf_idx = args.iter().position(|a| *a == "-crf").expect("missing -crf flag");
        assert!(crf_idx + 1 < args.len(), "-crf must be followed by a value");
    }

    #[test]
    fn h265_4k_preset_returns_libx265_codec_chain() {
        let args = ffmpeg_args_for_preset("h265_4k").unwrap();
        assert!(args.contains(&"libx265"), "preset must use libx265 codec");
        assert!(args.contains(&"scale=-2:2160"), "preset must scale to 4K");
        assert!(args.contains(&"aac"), "preset must transcode audio to aac");
        let crf_idx = args.iter().position(|a| *a == "-crf").expect("missing -crf flag");
        assert!(crf_idx + 1 < args.len(), "-crf must be followed by a value");
    }

    #[test]
    fn audio_only_preset_returns_libmp3lame_codec() {
        let args = ffmpeg_args_for_preset("audio_only").unwrap();
        assert!(args.contains(&"libmp3lame"), "preset must use libmp3lame codec");
        assert!(args.contains(&"-vn"), "preset must disable video with -vn");
        assert!(args.contains(&"320k"), "preset must use 320k audio bitrate");
    }
}
