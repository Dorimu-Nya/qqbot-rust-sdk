use crate::events::c2c::event::{C2cEvent, C2cEventKind};
use crate::events::group::event::{GroupEvent, GroupEventKind};
use crate::events::guild::event::{ForumEvent, ForumEventKind, GuildEvent, GuildEventKind};
use crate::events::interaction::event::{InteractionEvent, InteractionEventKind};
use crate::events::message_reaction::event::{MessageReactionEvent, MessageReactionEventKind};
use serde::{Deserialize, Serialize};

/// 全部事件类型汇总分类
///
/// 以下的值根据回调配置页 <https://q.qq.com/qqbot/#/developer/webhook-setting> 提取做了分类
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Event {
    /// 单聊事件
    C2cEvent(C2cEvent),
    /// 群事件
    GroupEvent(GroupEvent),
    /// 频道事件
    GuildEvent(GuildEvent),
    /// 频道论坛事件
    ForumEvent(ForumEvent),
    /// 互动事件
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
