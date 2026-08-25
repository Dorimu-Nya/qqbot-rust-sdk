use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDirectMessageRequest {
    pub recipient_id: String,
    pub source_guild_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateDirectMessageResponse {
    #[serde(default)]
    pub guild_id: Option<String>,
    #[serde(default)]
    pub channel_id: Option<String>,
    #[serde(default)]
    pub create_time: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

pub async fn create_direct_message(
    _body: CreateDirectMessageRequest,
) -> CreateDirectMessageResponse {
    todo!()
}
