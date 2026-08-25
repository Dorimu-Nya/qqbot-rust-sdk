use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Channel {
    pub id: String,
    #[serde(default)]
    pub guild_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    pub kind: Option<i32>,
    #[serde(default)]
    pub position: Option<i32>,
    #[serde(default)]
    pub parent_id: Option<String>,
    #[serde(default)]
    pub owner_id: Option<String>,
    #[serde(default)]
    pub sub_type: Option<i32>,
    #[serde(default)]
    pub private_type: Option<i32>,
    #[serde(default)]
    pub speak_permission: Option<i32>,
    #[serde(default)]
    pub application_id: Option<String>,
}
