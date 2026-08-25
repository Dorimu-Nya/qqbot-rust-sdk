use serde::{Deserialize, Serialize};

use super::{ActionAnchor, ActionType, Permission};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Action {
    #[serde(rename = "type")]
    pub action_type: ActionType,
    pub permission: Permission,
    pub data: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enter: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anchor: Option<ActionAnchor>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub click_limit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub at_bot_show_channel_list: Option<bool>,
    pub unsupport_tips: String,
}
