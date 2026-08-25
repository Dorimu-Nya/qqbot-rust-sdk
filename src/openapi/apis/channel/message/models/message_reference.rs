use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MessageReference {
    #[serde(default)]
    pub message_id: Option<String>,
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}
