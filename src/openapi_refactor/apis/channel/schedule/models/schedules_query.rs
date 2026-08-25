use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SchedulesQuery {
    #[serde(default)]
    pub since: Option<u64>,
}
