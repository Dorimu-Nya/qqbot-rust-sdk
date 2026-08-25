use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DeleteMessageOptions {
    #[serde(default)]
    pub hidetip: Option<bool>,
}
