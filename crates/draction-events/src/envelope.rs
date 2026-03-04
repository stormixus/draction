use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Envelope {
    pub channel: String,
    pub payload: serde_json::Value,
}
