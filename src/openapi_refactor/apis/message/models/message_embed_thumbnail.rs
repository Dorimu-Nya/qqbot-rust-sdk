use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageEmbedThumbnail {
    pub url: Option<String>,
}
