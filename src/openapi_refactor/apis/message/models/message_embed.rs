use serde::{Deserialize, Serialize};

use super::{MessageEmbedField, MessageEmbedThumbnail};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageEmbed {
    pub title: Option<String>,
    pub prompt: Option<String>,
    pub thumbnail: Option<MessageEmbedThumbnail>,
    pub fields: Option<Vec<MessageEmbedField>>,
}
