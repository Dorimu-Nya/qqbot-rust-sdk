use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageMarkdownParam {
    pub key: String,
    pub values: Vec<String>,
}
