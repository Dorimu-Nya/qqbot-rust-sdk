use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduleInput {
    pub name: String,
    pub start_timestamp: String,
    pub end_timestamp: String,
    pub jump_channel_id: String,
    pub remind_type: String,
}
