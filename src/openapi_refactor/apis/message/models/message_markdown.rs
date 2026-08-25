use serde::{Deserialize, Serialize};

use super::keyboard::Keyboard;
use super::message_markdown_param::MessageMarkdownParam;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageMarkdown {
    pub content: Option<String>,
    pub custom_template_id: Option<String>,
    pub params: Option<Vec<MessageMarkdownParam>>,
    #[serde(skip)]
    pub keyboard: Option<Keyboard>,
}
