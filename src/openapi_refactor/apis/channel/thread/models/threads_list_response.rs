use serde::{Deserialize, Serialize};

use super::Thread;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreadsListResponse {
    pub threads: Vec<Thread>,
    pub is_finish: u32,
}
