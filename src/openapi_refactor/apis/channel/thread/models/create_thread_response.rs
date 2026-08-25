use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateThreadResponse {
    pub task_id: String,
    pub create_time: String,
}
