//! 所有具体事件变体的强类型标记。

/// 单聊事件标记。
pub use crate::events::c2c::event::c2c_event_markers as c2c;
/// 群聊事件标记。
pub use crate::events::group::event::group_event_markers as group;
/// 频道论坛事件标记。
pub use crate::events::guild::event::forum_event_markers as forum;
/// 频道事件标记。
pub use crate::events::guild::event::guild_event_markers as guild;
/// 互动事件标记。
pub use crate::events::interaction::event::interaction_event_markers as interaction;
/// 消息表情表态事件标记。
pub use crate::events::message_reaction::event::message_reaction_event_markers as message_reaction;
