use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Role {
    pub id: String,
    pub name: String,
    pub color: u32,
    pub hoist: i32,
    #[serde(default)]
    pub number: Option<u32>,
    #[serde(default)]
    pub member_limit: Option<u32>,
}
