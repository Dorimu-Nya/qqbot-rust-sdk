use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GuildMuteRequest {
    #[serde(default)]
    pub mute_end_timestamp: Option<String>,
    #[serde(default)]
    pub mute_seconds: Option<String>,
}
