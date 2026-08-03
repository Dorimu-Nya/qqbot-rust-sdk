use super::super::common::Attachment;
use serde::{Deserialize, Serialize};
use crate::events::common::ARKData;

#[derive(Debug, Clone, Serialize, Deserialize)]
/// 单聊消息作者
///
/// 参考: <https://bot.q.qq.com/wiki/develop/api-v2/autogen/event/c2c_message_create.html#schema-user>
pub struct C2cUser {
    /// 作者id
    pub id: Option<String>,
    /// 用户openid
    pub user_openid: String,
    /// 联合openid
    pub union_openid: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// 消息场景
///
/// 参考: <https://bot.q.qq.com/wiki/develop/api-v2/autogen/event/c2c_message_create.html#schema-messagescene>
pub struct C2cMessageScene {
    /// 场景来源。default=默认聊天窗口
    pub source: Option<String>,
    /// 扩展数据列表，key=value 格式: msg_idx=消息索引, 用于引用场景 ref_msg_idx=引用的消息索引 auth_token=鉴权令牌
    pub ext: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// 单聊消息事件
///
/// 参考: <https://bot.q.qq.com/wiki/develop/api-v2/autogen/event/c2c_message_create.html#%E4%BA%8B%E4%BB%B6>
pub struct C2cMessage {
    /// 平台方消息ID，可以用于被动消息发送
    pub id: String,
    /// 发送者
    pub author: C2cUser,
    /// 文本消息内容
    pub content: Option<String>,
    /// 消息生产时间（RFC3339）
    pub timestamp: Option<String>,
    /// 消息类型
    pub message_type: Option<u32>,
    /// 场景信息
    pub message_scene: Option<C2cMessageScene>,
    /// 结构化卡片消息数据（message_type=3 时有值）
    pub ark_data: Option<ARKData>,
    /// 富媒体文件附件，文件类型："图片，语音，视频，文件"
    pub attachments: Option<Vec<Attachment>>,
    #[serde(default)]
    /// 消息序列
    pub msg_seq: Option<u64>,
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
/// 参考: https://bot.q.qq.com/wiki/develop/api-v2/autogen/event/c2c_msg_receive.html
pub struct C2cMsgReceiveEvent {
    /// 操作时间戳
    pub timestamp: i64,
    /// 用户openid
    pub openid: String,
}
