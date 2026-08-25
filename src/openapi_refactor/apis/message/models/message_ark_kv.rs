use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageArkKv {
    pub key: String,
    pub value: String,
}
