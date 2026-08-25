use serde::{Deserialize, Serialize};

use super::RecommendChannel;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateAnnouncesRequest {
    #[serde(default)]
    pub message_id: Option<String>,
    #[serde(default)]
    pub channel_id: Option<String>,
    #[serde(default)]
    pub announces_type: Option<u32>,
    #[serde(default)]
    pub recommend_channels: Vec<RecommendChannel>,
}
