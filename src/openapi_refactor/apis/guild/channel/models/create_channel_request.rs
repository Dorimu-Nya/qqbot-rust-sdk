use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateChannelRequest {
    pub name: String,
    #[serde(rename = "type")]
    pub kind: i32,
    #[serde(default)]
    pub sub_type: Option<i32>,
    pub position: i32,
    #[serde(default)]
    pub parent_id: Option<String>,
    #[serde(default)]
    pub private_type: Option<i32>,
    #[serde(default)]
    pub private_user_ids: Vec<String>,
    #[serde(default)]
    pub speak_permission: Option<i32>,
    #[serde(default)]
    pub application_id: Option<String>,
}
