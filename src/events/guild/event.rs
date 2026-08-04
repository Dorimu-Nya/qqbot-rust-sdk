use super::audio::AudioOrLiveChannelMemberEvent;
use super::forum::{ForumEventAuditResult, ForumEventPost, ForumEventReply, ForumEventThread};
use super::guild::{ChannelData, GuildData};
use super::member::GuildMemberEvent;
use super::messages::GuildMessages;
use super::open_forum::OpenForumEvent;
use crate::events::event_kind;
use crate::events::payload::event::Event;
use crate::events::payload::payload::{DispatchPayload, FromDispatchPayload};
use serde::{Deserialize, Serialize};

event_kind!(
    /// 频道的事件类型列表
    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[serde(tag = "t", content = "d")]
    pub enum GuildEvent {
        /// 频道内 @ 机器人的消息事件
        #[serde(rename = "AT_MESSAGE_CREATE")]
        AtMessageCreate(GuildMessages),
        /// 撤回频道消息公域事件
        #[serde(rename = "PUBLIC_MESSAGE_DELETE")]
        PublicMessageDelete(),
        /// 私信创建事件
        #[serde(rename = "DIRECT_MESSAGE_CREATE")]
        DirectMessageCreate(GuildMessages),
        /// 频道私信删除事件
        #[serde(rename = "DIRECT_MESSAGE_DELETE")]
        DirectMessageDelete(),
        /// 为消息添加表情表态
        #[serde(rename = "MESSAGE_REACTION_ADD")]
        MessageReactionAdd,
        /// 为消息删除表情表态
        #[serde(rename = "MESSAGE_REACTION_REMOVE")]
        MessageReactionRemove,
        /// 频道内消息审核通过
        #[serde(rename = "MESSAGE_AUDIT_PASS")]
        MessageAuditPass(),
        /// 频道内消息审核不通过
        #[serde(rename = "MESSAGE_AUDIT_REJECT")]
        MessageAuditReject(),
        /// 公域论坛事件：用户创建主题
        #[serde(rename = "OPEN_FORUM_THREAD_CREATE")]
        OpenForumThreadCreate(OpenForumEvent),
        /// 公域论坛事件：用户创建帖子
        #[serde(rename = "OPEN_FORUM_POST_CREATE")]
        OpenForumPostCreate(OpenForumEvent),
        /// 公域论坛事件：用户回复帖子
        #[serde(rename = "OPEN_FORUM_REPLY_CREATE")]
        OpenForumReplyCreate(OpenForumEvent),
        /// 公域论坛事件：用户更新主题
        #[serde(rename = "OPEN_FORUM_THREAD_UPDATE")]
        OpenForumThreadUpdate(OpenForumEvent),
        /// 公域论坛事件：用户删除帖子
        #[serde(rename = "OPEN_FORUM_POST_DELETE")]
        OpenForumPostDelete(OpenForumEvent),
        /// 公域论坛事件：用户回复被删除
        #[serde(rename = "OPEN_FORUM_REPLY_DELETE")]
        OpenForumReplyDelete(OpenForumEvent),
        /// 公域论坛事件：用户删除主题
        #[serde(rename = "OPEN_FORUM_THREAD_DELETE")]
        OpenForumThreadDelete(OpenForumEvent),
        /// 频道创建事件
        #[serde(rename = "GUILD_CREATE")]
        GuildCreate(GuildData),
        /// 频道信息变更事件
        #[serde(rename = "GUILD_UPDATE")]
        GuildUpdate(GuildData),
        /// 频道删除事件
        #[serde(rename = "GUILD_DELETE")]
        GuildDelete(GuildData),
        /// 子频道创建事件
        #[serde(rename = "CHANNEL_CREATE")]
        ChannelCreate(ChannelData),
        /// 子频道修改事件
        #[serde(rename = "CHANNEL_UPDATE")]
        ChannelUpdate(ChannelData),
        /// 子频道删除事件
        #[serde(rename = "CHANNEL_DELETE")]
        ChannelDelete(ChannelData),
        /// 新成员加入频道事件
        #[serde(rename = "GUILD_MEMBER_ADD")]
        GuildMemberAdd(GuildMemberEvent),
        /// 频道成员离开频道事件
        #[serde(rename = "GUILD_MEMBER_REMOVE")]
        GuildMemberRemove(GuildMemberEvent),
        /// 频道成员信息更新
        #[serde(rename = "GUILD_MEMBER_UPDATE")]
        GuildMemberUpdate(GuildMemberEvent),
        /// 音频开始播放事件
        #[serde(rename = "AUDIO_START")]
        AudioStart(),
        /// 音频播放结束事件
        #[serde(rename = "AUDIO_FINISH")]
        AudioFinish(),
        /// 机器人上麦事件
        #[serde(rename = "AUDIO_ON_MIC")]
        AudioOnMic(),
        /// 机器人下麦事件
        #[serde(rename = "AUDIO_OFF_MIC")]
        AudioOffMic(),
        // 下面这两个在BOT后台webhook可订阅列表里没看见有，但是文档里的事件列表又有，不是很懂
        /// 音视频/直播子频道成员进事件
        #[serde(rename = "AUDIO_OR_LIVE_CHANNEL_MEMBER_ENTER")]
        AudioOrLiveChannelMemberEnter(AudioOrLiveChannelMemberEvent),
        /// 音视频/直播子频道成员出事件
        #[serde(rename = "AUDIO_OR_LIVE_CHANNEL_MEMBER_EXIT")]
        AudioOrLiveChannelMemberExit(AudioOrLiveChannelMemberEvent),
    }
);

// impl GuildEvent {
//     fn to_kind(self) -> GuildEventKind {
//         match self {
//             Self::AtMessageCreate(_) => GuildEventKind::AtMessageCreate,
//         }
//     }
// }

// 下面这一部分貌似没有用？webhook回调订阅好像没这些东西
event_kind!(
    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[serde(tag = "t", content = "d")]
    pub enum ForumEvent {
        /// 论坛事件：用户创建主题
        #[serde(rename = "FORUM_THREAD_CREATE")]
        ForumThreadCreate(ForumEventThread),
        /// 论坛事件：用户更新主题
        #[serde(rename = "FORUM_THREAD_UPDATE")]
        ForumThreadUpdate(ForumEventThread),
        /// 论坛事件：用户删除主题
        #[serde(rename = "FORUM_THREAD_DELETE")]
        ForumThreadDelete(ForumEventThread),
        /// 论坛事件：用户创建帖子
        #[serde(rename = "FORUM_POST_CREATE")]
        ForumPostCreate(ForumEventPost),
        /// 论坛事件：用户删除帖子
        #[serde(rename = "FORUM_POST_DELETE")]
        ForumPostDelete(ForumEventPost),
        /// 论坛事件：用户回复帖子
        #[serde(rename = "FORUM_REPLY_CREATE")]
        ForumReplyCreate(ForumEventReply),
        /// 论坛事件：用户回复被删除
        #[serde(rename = "FORUM_REPLY_DELETE")]
        ForumReplyDelete(ForumEventReply),
        /// 帖子审核事件
        #[serde(rename = "FORUM_PUBLISH_AUDIT_RESULT")]
        ForumAuditEvent(ForumEventAuditResult),
    }
);

impl FromDispatchPayload for GuildMessages {
    fn from(payload: &DispatchPayload) -> Option<Self> {
        match &payload.event {
            Event::GuildEvent(GuildEvent::AtMessageCreate(value))
            | Event::GuildEvent(GuildEvent::DirectMessageCreate(value)) => Some(value.clone()),
            _ => None,
        }
    }
}

impl FromDispatchPayload for OpenForumEvent {
    fn from(payload: &DispatchPayload) -> Option<Self> {
        match &payload.event {
            Event::GuildEvent(GuildEvent::OpenForumThreadCreate(value))
            | Event::GuildEvent(GuildEvent::OpenForumPostCreate(value))
            | Event::GuildEvent(GuildEvent::OpenForumReplyCreate(value))
            | Event::GuildEvent(GuildEvent::OpenForumThreadUpdate(value))
            | Event::GuildEvent(GuildEvent::OpenForumPostDelete(value))
            | Event::GuildEvent(GuildEvent::OpenForumReplyDelete(value))
            | Event::GuildEvent(GuildEvent::OpenForumThreadDelete(value)) => Some(value.clone()),
            _ => None,
        }
    }
}

impl FromDispatchPayload for GuildData {
    fn from(payload: &DispatchPayload) -> Option<Self> {
        match &payload.event {
            Event::GuildEvent(GuildEvent::GuildCreate(value))
            | Event::GuildEvent(GuildEvent::GuildUpdate(value))
            | Event::GuildEvent(GuildEvent::GuildDelete(value)) => Some(value.clone()),
            _ => None,
        }
    }
}

impl FromDispatchPayload for ChannelData {
    fn from(payload: &DispatchPayload) -> Option<Self> {
        match &payload.event {
            Event::GuildEvent(GuildEvent::ChannelCreate(value))
            | Event::GuildEvent(GuildEvent::ChannelUpdate(value))
            | Event::GuildEvent(GuildEvent::ChannelDelete(value)) => Some(value.clone()),
            _ => None,
        }
    }
}

impl FromDispatchPayload for GuildMemberEvent {
    fn from(payload: &DispatchPayload) -> Option<Self> {
        match &payload.event {
            Event::GuildEvent(GuildEvent::GuildMemberAdd(value))
            | Event::GuildEvent(GuildEvent::GuildMemberRemove(value))
            | Event::GuildEvent(GuildEvent::GuildMemberUpdate(value)) => Some(value.clone()),
            _ => None,
        }
    }
}

impl FromDispatchPayload for AudioOrLiveChannelMemberEvent {
    fn from(payload: &DispatchPayload) -> Option<Self> {
        match &payload.event {
            Event::GuildEvent(GuildEvent::AudioOrLiveChannelMemberEnter(value))
            | Event::GuildEvent(GuildEvent::AudioOrLiveChannelMemberExit(value)) => {
                Some(value.clone())
            }
            _ => None,
        }
    }
}

impl FromDispatchPayload for ForumEventThread {
    fn from(payload: &DispatchPayload) -> Option<Self> {
        match &payload.event {
            Event::ForumEvent(ForumEvent::ForumThreadCreate(value))
            | Event::ForumEvent(ForumEvent::ForumThreadUpdate(value))
            | Event::ForumEvent(ForumEvent::ForumThreadDelete(value)) => Some(value.clone()),
            _ => None,
        }
    }
}

impl FromDispatchPayload for ForumEventPost {
    fn from(payload: &DispatchPayload) -> Option<Self> {
        match &payload.event {
            Event::ForumEvent(ForumEvent::ForumPostCreate(value))
            | Event::ForumEvent(ForumEvent::ForumPostDelete(value)) => Some(value.clone()),
            _ => None,
        }
    }
}

impl FromDispatchPayload for ForumEventReply {
    fn from(payload: &DispatchPayload) -> Option<Self> {
        match &payload.event {
            Event::ForumEvent(ForumEvent::ForumReplyCreate(value))
            | Event::ForumEvent(ForumEvent::ForumReplyDelete(value)) => Some(value.clone()),
            _ => None,
        }
    }
}

impl FromDispatchPayload for ForumEventAuditResult {
    fn from(payload: &DispatchPayload) -> Option<Self> {
        match &payload.event {
            Event::ForumEvent(ForumEvent::ForumAuditEvent(value)) => Some(value.clone()),
            _ => None,
        }
    }
}
