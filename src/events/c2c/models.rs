use super::super::common::ARKData;
use super::super::common::MessageAttachment;
use super::super::common::MessageScene;
use crate::events::common::{C2cUser, MsgElement};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
/// 单聊消息事件
///
/// 参考: <https://bot.q.qq.com/wiki/develop/api-v2/autogen/event/c2c_message_create.html#%E4%BA%8B%E4%BB%B6>
pub struct C2cMessage {
    /// 消息 ID，可用于被动回复和撤回
    pub id: String,
    /// 发送者（user_openid 有值）
    pub author: C2cUser,
    /// 消息文本内容
    pub content: Option<String>,
    /// 消息发送时间，RFC3339 格式
    pub timestamp: Option<String>,
    /// 消息内容类型: 0=普通文本, 3=结构化卡片, 101=并行消息, 102=聊天记录, 103=引用消息
    pub message_type: Option<u32>,
    /// 消息场景上下文（含消息索引、鉴权令牌等）
    pub message_scene: Option<MessageScene>,
    /// 消息附件（图片、文件、语音等）
    pub attachments: Option<Vec<MessageAttachment>>,
    /// 结构化卡片消息数据（message_type=3 时有值）
    pub ark_data: Option<ARKData>,
    /// 消息元素列表（message_type=103 引用消息时包含被引用内容）
    pub msg_elements: Option<Vec<MsgElement>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// 用户添加好友
///
/// 参考: <https://bot.q.qq.com/wiki/develop/api-v2/autogen/event/friend_add.html#%E4%BA%8B%E4%BB%B6>
pub struct FriendAddEvent {
    /// 添加时间戳
    pub timestamp: i64,
    /// 用户openid
    pub openid: String,
    /// 场景 id
    pub scene: Option<i64>,
    /// 场景参数
    pub scene_param: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// 用户删除好友
///
/// 参考: <https://bot.q.qq.com/wiki/develop/api-v2/autogen/event/friend_del.html>
pub struct FriendDelEvent {
    /// 删除时间戳
    pub timestamp: i64,
    /// 用户openid
    pub openid: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// 单聊消息接收关闭
///
/// 参考: <https://bot.q.qq.com/wiki/develop/api-v2/autogen/event/c2c_msg_reject.html>
pub struct C2cMsgRejectEvent {
    /// 操作时间戳
    pub timestamp: i64,
    /// 用户openid
    pub openid: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// 单聊消息接收开启
///
/// 参考: <https://bot.q.qq.com/wiki/develop/api-v2/autogen/event/c2c_msg_receive.html>
pub struct C2cMsgReceiveEvent {
    /// 操作时间戳
    pub timestamp: i64,
    /// 用户openid
    pub openid: String,
}
