use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    // General
    pub launch_at_login: bool,
    pub show_draky_on_launch: bool,
    pub run_minimized: bool,
    pub theme: String,            // "system", "light", "dark"
    pub accent_color: String,     // hex color
    pub reduce_motion: bool,
    pub language: String,
    pub date_format: String,

    // Inbox
    pub inbox_location: String,
    pub conflict_resolution: String,  // "rename", "overwrite", "skip"
    pub delete_source_after_ingest: bool,

    // Rules Behavior
    pub match_policy: String,      // "first_wins", "all_matching"
    pub concurrency: u32,
    pub undo_window_seconds: u32,

    // Draky
    pub draky_size: String,        // "small", "medium", "large"
    pub draky_personality: String, // "professional", "playful", "zen"
    pub draky_always_on_top: bool,

    // Connections
    pub openclaw_paired: bool,
    pub openclaw_device_id: String,
    pub pairing_code: String,

    // Safety
    pub allowed_path_scopes: Vec<String>,
    pub dangerous_nodes_enabled: bool,
    pub require_confirmation_for: Vec<String>,

    // Advanced
    pub db_path: String,
    pub log_level: String,
    pub api_port: u16,
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
            date_format: "%Y-%m-%d %H:%M".into(),
            inbox_location: "~/Draction/Inbox".into(),
            conflict_resolution: "rename".into(),
            delete_source_after_ingest: false,
            match_policy: "first_wins".into(),
            concurrency: 1,
            undo_window_seconds: 10,
            draky_size: "medium".into(),
            draky_personality: "playful".into(),
            draky_always_on_top: true,
            openclaw_paired: false,
            openclaw_device_id: String::new(),
            pairing_code: String::new(),
            allowed_path_scopes: vec!["~/".into()],
            dangerous_nodes_enabled: false,
            require_confirmation_for: vec!["transcode".into(), "webhook".into()],
            db_path: String::new(),
            log_level: "info".into(),
            api_port: 9400,
        }
    }
}

impl Settings {
    pub fn load(base_dir: &std::path::Path) -> anyhow::Result<Self> {
        let path = base_dir.join("settings.json");
        if path.exists() {
            let data = std::fs::read_to_string(&path)?;
            Ok(serde_json::from_str(&data).unwrap_or_default())
        } else {
            let settings = Self::default();
            settings.save(base_dir)?;
            Ok(settings)
        }
    }

    pub fn save(&self, base_dir: &std::path::Path) -> anyhow::Result<()> {
        let path = base_dir.join("settings.json");
        std::fs::write(&path, serde_json::to_string_pretty(self)?)?;
        Ok(())
    }
}
