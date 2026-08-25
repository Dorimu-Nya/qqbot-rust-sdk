use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageEmbedField {
    pub name: Option<String>,
    pub value: Option<String>,
}
