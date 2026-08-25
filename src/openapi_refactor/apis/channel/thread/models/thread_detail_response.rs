use serde::{Deserialize, Serialize};

use super::Thread;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreadDetailResponse {
    pub thread: Thread,
}
