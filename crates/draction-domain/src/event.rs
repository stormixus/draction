use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IngestEvent {
    pub id: String,
    pub time: DateTime<Utc>,
    pub source: EventSource,
    pub files: Vec<IngestFile>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EventSource {
    pub kind: String,
    pub device_name: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IngestFile {
    pub path: String,
    pub name: String,
    pub ext: Option<String>,
    pub size_bytes: u64,
    pub mime: Option<String>,
    pub sha256: Option<String>,
    pub is_folder: bool,
}
