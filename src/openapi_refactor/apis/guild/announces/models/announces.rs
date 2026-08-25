use serde::{Deserialize, Serialize};

use super::recommend_channel::RecommendChannel;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Announces {
    #[serde(default)]
    pub guild_id: Option<String>,
    #[serde(default)]
    pub channel_id: Option<String>,
    #[serde(default)]
    pub message_id: Option<String>,
    #[serde(default)]
    pub announces_type: Option<u32>,
    #[serde(default)]
    pub recommend_channels: Vec<RecommendChannel>,
}
