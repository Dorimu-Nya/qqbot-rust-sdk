use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GroupMembersQuery {
    #[serde(default)]
    pub limit: Option<u32>,
    #[serde(default)]
    pub start_index: Option<u32>,
}
