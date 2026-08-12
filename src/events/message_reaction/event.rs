use super::reaction::MessageReaction;
use crate::events::event_kind;
use serde::{Deserialize, Serialize};

event_kind!(
    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[serde(tag = "t", content = "d")]
    /// 消息表情表态的事件类型列表
    pub enum MessageReactionEvent {
        /// 为消息添加表情表态
        #[serde(rename = "MESSAGE_REACTION_ADD")]
        MessageReactionAdd(MessageReaction),
        /// 为消息删除表情表态
        #[serde(rename = "MESSAGE_REACTION_REMOVE")]
        MessageReactionRemove(MessageReaction),
    }
);
