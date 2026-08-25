use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Guild {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub icon: Option<String>,
    #[serde(default)]
    pub owner_id: Option<String>,
    #[serde(default)]
    pub owner: Option<bool>,
    #[serde(default)]
    pub joined_at: Option<String>,
    #[serde(default)]
    pub member_count: Option<u32>,
    #[serde(default)]
    pub max_members: Option<u32>,
    #[serde(default)]
    pub description: Option<String>,
}
