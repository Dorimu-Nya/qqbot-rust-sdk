use serde::{Deserialize, Serialize};

use super::json_object::JsonObject;
use super::keyboard::Keyboard;
use super::message_ark::MessageArk;
use super::message_embed::MessageEmbed;
use super::message_markdown::MessageMarkdown;
use super::message_media::MessageMedia;
use super::message_type::MessageType;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendMessageRequest {
    #[serde(default)]
    pub content: Option<String>,
    pub msg_type: MessageType,
    #[serde(default)]
    pub markdown: Option<MessageMarkdown>,
    #[serde(default)]
    pub keyboard: Option<Keyboard>,
    #[serde(default)]
    pub ark: Option<MessageArk>,
    #[serde(default)]
    pub media: Option<MessageMedia>,
    #[serde(default)]
    pub embed: Option<MessageEmbed>,
    #[serde(default)]
    pub message_reference: Option<JsonObject>,
    #[serde(default)]
    pub event_id: Option<String>,
    #[serde(default)]
    pub msg_id: Option<String>,
    #[serde(default)]
    pub msg_seq: Option<u64>,
}
