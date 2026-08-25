use serde::{Deserialize, Serialize};

use super::message_embed_field::MessageEmbedField;
use super::message_embed_thumbnail::MessageEmbedThumbnail;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageEmbed {
    pub title: Option<String>,
    pub prompt: Option<String>,
    pub thumbnail: Option<MessageEmbedThumbnail>,
    pub fields: Option<Vec<MessageEmbedField>>,
}
