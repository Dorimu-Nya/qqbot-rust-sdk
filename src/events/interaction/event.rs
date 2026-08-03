use super::models::Interaction;
use crate::events::payload::event::Event;
use crate::events::event_kind;
use crate::events::payload::payload::{DispatchPayload, FromDispatchPayload};
use serde::{Deserialize, Serialize};

event_kind!(
    /// 互动事件
    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[serde(tag = "t", content = "d")]
    pub enum InteractionEvent {
        /// 创建互动事件
        #[serde(rename = "INTERACTION_CREATE")]
        InteractionCreate(Interaction),
    }
);

impl FromDispatchPayload for Interaction {
    fn from(payload: &DispatchPayload) -> Option<Self> {
        match &payload.event {
            Event::InteractionEvent(InteractionEvent::InteractionCreate(value)) => {
                Some(value.clone())
            }
            _ => None,
        }
    }
}
