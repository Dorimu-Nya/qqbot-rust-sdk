use super::super::common::MessageAttachment;
use crate::events::common::{ARKData, GroupUser, MessageScene, MsgElement};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
/// 群消息
///
/// at机器人和全量消息都是这个数据结构
///
/// 参考1: <https://bot.q.qq.com/wiki/develop/api-v2/autogen/event/group_at_message_create.html>
///
/// 参考2: <https://bot.q.qq.com/wiki/develop/api-v2/autogen/event/group_message_create.html>
pub struct GroupMessage {
    /// 平台方消息ID，可以用于被动消息发送
    pub id: String,
    /// 发送者
    pub author: GroupUser,
    /// 消息文本内容（已去除@机器人的前缀）
    pub content: Option<String>,
    /// 群 OpenID
    pub group_openid: String,
    /// 消息发送时间，RFC3339 格式
    pub timestamp: Option<String>,
    /// 消息内容类型
    pub message_type: i32,
    /// 消息场景上下文
    pub message_scene: MessageScene,
    /// 富媒体文件附件，文件类型："图片，语音，视频，文件"
    pub attachments: Option<Vec<MessageAttachment>>,
    /// 消息中@的用户列表
    pub mentions: Option<Vec<GroupMention>>,
    /// 结构化卡片消息数据
    pub ark_data: Option<ARKData>,
    /// 消息元素列表
    pub msg_elements: Option<Vec<MsgElement>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MentionScopeSingle {
    Single,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MentionScopeAll {
    All,
}
/// 被 @ 时的数据结构
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GroupMention {
    Single(GroupMentionUser),
    All(GroupMentionAll),
}

/// @单个用户时的结构体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupMentionUser {
    /// 通用用户信息
    #[serde(flatten)]
    pub user: GroupUser,
    scope: MentionScopeSingle,
    is_you: bool,
}

/// @全体成员 时的结构体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupMentionAll {
    username: String,
    scope: MentionScopeAll,
    is_you: bool,
}
// 吐槽一下上面这些文档里没有

#[derive(Debug, Clone, Serialize, Deserialize)]
/// 机器人加入群聊
///
/// 参考: <https://bot.q.qq.com/wiki/develop/api-v2/autogen/event/group_add_robot.html>
pub struct GroupAddRobotEvent {
    /// 操作时间戳
    pub timestamp: i64,
    /// 群 openid
    pub group_openid: String,
    /// 操作成员 openid
    pub op_member_openid: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// 机器人退出群聊
///
/// 参考: <https://bot.q.qq.com/wiki/develop/api-v2/autogen/event/group_del_robot.html>
pub struct GroupDelRobotEvent {
    /// 操作时间戳
    pub timestamp: i64,
    /// 群 openid
    pub group_openid: String,
    /// 操作成员 openid
    pub op_member_openid: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// 群聊消息接收关闭
///
/// 参考: <https://bot.q.qq.com/wiki/develop/api-v2/autogen/event/group_msg_reject.html>
pub struct GroupMsgRejectEvent {
    /// 操作时间戳
    pub timestamp: i64,
    /// 群 openid
    pub group_openid: String,
    /// 操作成员 openid
    pub op_member_openid: String,
}
