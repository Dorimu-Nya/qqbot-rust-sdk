use serde::{Deserialize, Serialize};

use super::thread_format::ThreadFormat;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateThreadRequest {
    pub title: String,
    pub content: String,
    pub format: ThreadFormat,
}
