use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::super::super::super::super::models::member::Member;
use super::super::super::super::super::models::user::User;
use super::message_attachment::MessageAttachment;
use super::message_reference::MessageReference;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Message {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub channel_id: Option<String>,
    #[serde(default)]
    pub guild_id: Option<String>,
    #[serde(default)]
    pub content: Option<String>,
    #[serde(default)]
    pub timestamp: Option<String>,
    #[serde(default)]
    pub edited_timestamp: Option<String>,
    #[serde(default)]
    pub author: Option<User>,
    #[serde(default)]
    pub member: Option<Member>,
    #[serde(default)]
    pub message_reference: Option<MessageReference>,
    #[serde(default)]
    pub mentions: Vec<User>,
    #[serde(default)]
    pub attachments: Vec<MessageAttachment>,
    #[serde(default)]
    pub seq: Option<u64>,
    #[serde(default)]
    pub seq_in_channel: Option<String>,
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}
