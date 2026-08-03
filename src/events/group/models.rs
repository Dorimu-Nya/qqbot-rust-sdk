use super::super::common::Attachment;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
/// 群消息作者
/// 
/// 参考: <https://bot.q.qq.com/wiki/develop/api-v2/autogen/event/group_message_create.html#schema-user>
pub struct GroupAuthor {
    /// 成员 openid
    pub member_openid: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// 群 @ 消息
///
/// 参考: <https://bot.q.qq.com/wiki/develop/api-v2/autogen/event/group_at_message_create.html>
pub struct GroupAtMessage {
    /// 平台方消息ID，可以用于被动消息发送
    pub id: String,
    /// 发送者
    pub author: GroupAuthor,
    /// 文本消息内容
    pub content: Option<String>,
    /// 消息生产时间（RFC3339）
    pub timestamp: Option<String>,
    /// 群 openid
    pub group_openid: String,
    /// 富媒体文件附件，文件类型："图片，语音，视频，文件"
    pub attachments: Option<Vec<Attachment>>,
    #[serde(default)]
    /// 消息序列
    pub msg_seq: Option<u64>,
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
