use serde::{Deserialize, Serialize};

use super::super::super::super::message::models::keyboard::Keyboard;
use super::super::super::super::message::models::message_markdown::MessageMarkdown;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateMessageRequest {
    #[serde(default)]
    pub msg_id: Option<String>,
    #[serde(default)]
    pub event_id: Option<String>,
    #[serde(default)]
    pub markdown: Option<MessageMarkdown>,
    #[serde(default)]
    pub keyboard: Option<Keyboard>,
}
