use serde::{Deserialize, Serialize};

use super::ScheduleInput;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpsertScheduleRequest {
    pub schedule: ScheduleInput,
}
