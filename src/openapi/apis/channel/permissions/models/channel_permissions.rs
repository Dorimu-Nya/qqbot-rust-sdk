use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelPermissions {
    pub channel_id: String,
    #[serde(default)]
    pub user_id: Option<String>,
    #[serde(default)]
    pub role_id: Option<String>,
    pub permissions: String,
}
