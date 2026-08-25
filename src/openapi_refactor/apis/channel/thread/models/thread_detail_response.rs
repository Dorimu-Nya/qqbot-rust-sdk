use serde::{Deserialize, Serialize};

use super::thread::Thread;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreadDetailResponse {
    pub thread: Thread,
}
