use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RoleMembersQuery {
    #[serde(default)]
    pub start_index: Option<String>,
    #[serde(default)]
    pub limit: Option<u32>,
}
