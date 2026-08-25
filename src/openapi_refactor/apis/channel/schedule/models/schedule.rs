use serde::{Deserialize, Serialize};

use super::super::super::super::super::models::member::Member;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Schedule {
    pub id: String,
    pub name: String,
    pub start_timestamp: String,
    pub end_timestamp: String,
    #[serde(default)]
    pub creator: Option<Member>,
    pub jump_channel_id: String,
    pub remind_type: String,
}
