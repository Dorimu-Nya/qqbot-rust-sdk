use serde::{Deserialize, Serialize};

use super::thread::Thread;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreadsListResponse {
    pub threads: Vec<Thread>,
    pub is_finish: u32,
}
