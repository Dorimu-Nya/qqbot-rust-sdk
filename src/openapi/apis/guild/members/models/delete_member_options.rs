use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DeleteMemberOptions {
    #[serde(default)]
    pub add_blacklist: Option<bool>,
    #[serde(default)]
    pub delete_history_msg_days: Option<i32>,
}
