use serde::{Deserialize, Serialize};

use super::schedule_input::ScheduleInput;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpsertScheduleRequest {
    pub schedule: ScheduleInput,
}
