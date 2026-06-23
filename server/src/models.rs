use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameEvent {
    pub r#type: String,
    pub data: serde_json::Value,
}