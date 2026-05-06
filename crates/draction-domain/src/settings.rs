use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
    // General
    pub launch_at_login: bool,
    pub show_draky_on_launch: bool,
    pub run_minimized: bool,
    pub theme: String,        // "system", "light", "dark"
    pub accent_color: String, // hex color
    pub reduce_motion: bool,
    pub language: String,
    pub date_format: String,

    // Inbox
    pub inbox_location: String,
    pub conflict_resolution: String, // "rename", "overwrite", "skip"
    pub delete_source_after_ingest: bool,
    pub date_subfolders: bool,
    pub inbox_size_limit_gb: u32,
    pub auto_archive_days: u32,
    pub undo_history_depth: u32,

    // Rules Behavior
    pub match_policy: String, // "first_wins", "all_matching"
    pub concurrency: u32,
    pub undo_window_seconds: u32,
    pub unmatched_files_policy: String,
    pub node_failure_policy: String,
    pub auto_retry: bool,
    pub keep_partial_artifacts: bool,
    pub toast_on_success: bool,
    pub toast_on_failure: bool,
    pub notification_sound: String,

    // Draky
    pub draky_size: String,        // "small", "medium", "large"
    pub draky_personality: String, // "professional", "playful", "zen"
    pub draky_always_on_top: bool,
    pub draky_overlay_visible: bool,
    pub draky_snap_to_corner: bool,
    pub draky_burp_on_success: bool,
    pub draky_idle_behaviors: bool,
    pub draky_file_type_munch: bool,

    // Connections
    pub openclaw_paired: bool,
    pub openclaw_device_id: String,
    pub pairing_code: String,
    pub api_bind_address: String,
    pub openclaw_auto_suggest_rules: bool,
    pub openclaw_send_file_metadata: bool,

    // Safety
    pub allowed_path_scopes: Vec<String>,
    pub dangerous_nodes_enabled: bool,
    pub allowed_dangerous_nodes: Vec<String>,
    pub require_confirmation_for: Vec<String>,

    // Advanced
    pub db_path: String,
    pub rules_path: String,
    pub workflows_path: String,
    pub log_level: String,
    pub log_retention_days: u32,
    pub api_port: u16,
    pub max_file_size_mb: u64,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            launch_at_login: false,
            show_draky_on_launch: true,
            run_minimized: false,
            theme: "dark".into(),
            accent_color: "#2dd4bf".into(),
            reduce_motion: false,
            language: "en".into(),
            date_format: "YYYY-MM-DD".into(),
            inbox_location: "~/Draction/Inbox".into(),
            conflict_resolution: "append_timestamp".into(),
            delete_source_after_ingest: false,
            date_subfolders: true,
            inbox_size_limit_gb: 10,
            auto_archive_days: 30,
            undo_history_depth: 5,
            match_policy: "first_match".into(),
            concurrency: 1,
            undo_window_seconds: 10,
            unmatched_files_policy: "keep_inbox".into(),
            node_failure_policy: "fail_fast".into(),
            auto_retry: false,
            keep_partial_artifacts: true,
            toast_on_success: true,
            toast_on_failure: true,
            notification_sound: "off".into(),
            draky_size: "medium".into(),
            draky_personality: "friendly".into(),
            draky_always_on_top: true,
            draky_overlay_visible: true,
            draky_snap_to_corner: true,
            draky_burp_on_success: true,
            draky_idle_behaviors: true,
            draky_file_type_munch: true,
            openclaw_paired: false,
            openclaw_device_id: String::new(),
            pairing_code: "DRK-7Q2-91A".into(),
            api_bind_address: "127.0.0.1".into(),
            openclaw_auto_suggest_rules: false,
            openclaw_send_file_metadata: true,
            allowed_path_scopes: vec!["~/Draction/Inbox".into(), "~/Draction/Work".into()],
            dangerous_nodes_enabled: false,
            allowed_dangerous_nodes: Vec::new(),
            require_confirmation_for: vec!["system_folders".into(), "batch_ingest".into()],
            db_path: "~/Draction/draction.db".into(),
            rules_path: "~/Draction/rules.json".into(),
            workflows_path: "~/Draction/workflows.json".into(),
            log_level: "info".into(),
            log_retention_days: 14,
            api_port: 9400,
            max_file_size_mb: 500,
        }
    }
}

impl Settings {
    pub async fn load(base_dir: &std::path::Path) -> anyhow::Result<Self> {
        let path = base_dir.join("settings.json");
        if path.exists() {
            let data = tokio::fs::read_to_string(&path).await?;
            Ok(serde_json::from_str(&data).unwrap_or_default())
        } else {
            let settings = Self::default();
            settings.save(base_dir).await?;
            Ok(settings)
        }
    }

    pub async fn save(&self, base_dir: &std::path::Path) -> anyhow::Result<()> {
        let path = base_dir.join("settings.json");
        tokio::fs::write(&path, serde_json::to_string_pretty(self)?).await?;
        Ok(())
    }
}
