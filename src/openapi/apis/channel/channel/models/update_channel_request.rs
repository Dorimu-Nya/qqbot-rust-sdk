use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateChannelRequest {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub position: Option<i32>,
    #[serde(default)]
    pub parent_id: Option<String>,
    #[serde(default)]
    pub private_type: Option<i32>,
    #[serde(default)]
    pub speak_permission: Option<i32>,
}
