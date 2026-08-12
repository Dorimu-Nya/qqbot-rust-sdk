use super::models::Interaction;
use crate::events::event_kind;
use serde::{Deserialize, Serialize};

event_kind!(
    /// 互动事件的事件类型列表
    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[serde(tag = "t", content = "d")]
    pub enum InteractionEvent {
        /// 创建互动事件
        #[serde(rename = "INTERACTION_CREATE")]
        InteractionCreate(Interaction),
    }
);
