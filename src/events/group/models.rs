use super::super::common::MessageAttachment;
use crate::events::common::{ARKData, GroupUser, MessageScene, MsgElement, User};
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
    pub mentions: Option<Vec<User>>,
    /// 结构化卡片消息数据
    pub ark_data: Option<ARKData>,
    /// 消息元素列表
    pub msg_elements: Option<MsgElement>,
}

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
