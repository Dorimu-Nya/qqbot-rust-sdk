use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PinsMessage {
    pub guild_id: String,
    pub channel_id: String,
    pub message_ids: Vec<String>,
}
