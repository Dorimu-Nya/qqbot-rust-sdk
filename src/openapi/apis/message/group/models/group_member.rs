use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GroupMember {
    #[serde(default)]
    pub member_openid: Option<String>,
    #[serde(default)]
    pub join_timestamp: Option<u64>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}
