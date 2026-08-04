use super::reaction::MessageReaction;
use crate::events::event_kind;
use crate::events::payload::event::Event;
use crate::events::payload::payload::{DispatchPayload, FromDispatchPayload};
use serde::{Deserialize, Serialize};

event_kind!(
    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[serde(tag = "t", content = "d")]
    pub enum MessageReactionEvent {
        /// 为消息添加表情表态
        #[serde(rename = "MESSAGE_REACTION_ADD")]
        MessageReactionAdd(MessageReaction),
        /// 为消息删除表情表态
        #[serde(rename = "MESSAGE_REACTION_REMOVE")]
        MessageReactionRemove(MessageReaction),
    }
);

impl FromDispatchPayload for MessageReaction {
    fn from(payload: &DispatchPayload) -> Option<Self> {
        match &payload.event {
            Event::MessageReactionEvent(MessageReactionEvent::MessageReactionAdd(value))
            | Event::MessageReactionEvent(MessageReactionEvent::MessageReactionRemove(value)) => {
                Some(value.clone())
            }
            _ => None,
        }
    }
}
