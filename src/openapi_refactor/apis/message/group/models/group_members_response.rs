use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use super::GroupMember;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GroupMembersResponse {
    #[serde(default)]
    pub members: Vec<GroupMember>,
    #[serde(default)]
    pub next_index: Option<u32>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}
