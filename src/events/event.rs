use super::c2c::event::{C2cEvent, C2cEventKind};
use super::group::event::{GroupEvent, GroupEventKind};
use super::guild::event::{ForumEvent, ForumEventKind, GuildEvent, GuildEventKind};
use super::interaction::event::{InteractionEvent, InteractionEventKind};
use super::message_reaction::event::{MessageReactionEvent, MessageReactionEventKind};
use serde::{Deserialize, Serialize};

/// 全部事件类型汇总分类
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Event {
    C2cEvent(C2cEvent),
    GroupEvent(GroupEvent),
    GuildEvent(GuildEvent),
    ForumEvent(ForumEvent),
    InteractionEvent(InteractionEvent),
    MessageReactionEvent(MessageReactionEvent),
}

/// 全部事件注册键的统一表示。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EventKind {
    C2c(C2cEventKind),
    Group(GroupEventKind),
    Guild(GuildEventKind),
    Forum(ForumEventKind),
    Interaction(InteractionEventKind),
    MessageReaction(MessageReactionEventKind),
}

macro_rules! impl_event_kind_from {
    ($kind:ty, $variant:ident) => {
        impl From<$kind> for EventKind {
            fn from(kind: $kind) -> Self {
                Self::$variant(kind)
            }
        }
    };
}

impl_event_kind_from!(C2cEventKind, C2c);
impl_event_kind_from!(GroupEventKind, Group);
impl_event_kind_from!(GuildEventKind, Guild);
impl_event_kind_from!(ForumEventKind, Forum);
impl_event_kind_from!(InteractionEventKind, Interaction);
impl_event_kind_from!(MessageReactionEventKind, MessageReaction);
