use super::event::Event;
use super::opcode::{
    DispatchOp, HeartbeatACK, Hello, HttpCallbackAckOp, Identify, Resume, WebhookAddressVerifyOp,
};
use crate::events::validation::ValidationRequest;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
/// 通用数据结构的基础封装
///
/// 参考: <https://bot.q.qq.com/wiki/develop/api-v2/dev-prepare/event-emit/payload.html#%E9%80%9A%E7%94%A8%E6%95%B0%E6%8D%AE%E7%BB%93%E6%9E%84>
pub enum WebhookPayload {
    // opcode=0
    /// 服务端进行消息推送
    Dispatch(DispatchPayload),
    // opcode = 12
    /// http 回调模式的回包
    HttpCallbackAck(HttpCallbackAckPayload),
    // opcode=13
    /// 开放平台对机器人服务端进行验证
    WebhookAddressVerify(WebhookAddressVerifyPayload),
}

/// opcode为0时，服务端进行消息推送的消息对象
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DispatchPayload {
    /// 事件id
    pub id: Option<String>,
    ///  下行消息都会有一个序列号，标识消息的唯一性，客户端需要再发送心跳的时候，携带客户端收到的最新的s
    pub s: Option<u64>,
    /// 代表事件类型
    pub op: DispatchOp,
    /// 代表事件内容，不同事件类型的事件内容格式都不同，请注意识别。主要用在op为 0 Dispatch 的时候
    #[serde(flatten)]
    pub event: Event,
}

/// opcode为2时，websocket 登录所发送的包 用于获得session
#[derive(Debug, Serialize, Deserialize)]
pub struct IdentifyPayload {
    pub op: Identify,
    pub d: IdentifyData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdentifyData {
    pub token: String,
    pub indents: u64,
    pub shard: (u8, u8),
    pub properties: Option<HashMap<String, String>>,
}

/// opcode为6时，websocket 恢复所发送的包
#[derive(Debug, Serialize, Deserialize)]
pub struct ResumePayload {
    pub op: Resume,
    pub d: ResumeData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResumeData {
    pub token: String,
    pub session_id: String,
    pub seq: u64,
}

/// opcode为10时 所接收的包
#[derive(Debug, Serialize, Deserialize)]
pub struct HelloPayload {
    pub op: Hello,
    pub d: HelloData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HelloData {
    pub heartbeat_interval: u64,
}

/// opcode为11时，心跳所需要的回应
#[derive(Debug, Serialize, Deserialize)]
pub struct HeartbeatAckPayload {
    pub op: HeartbeatACK,
}

/// opcode为12时，http 回调模式的回包
#[derive(Debug, Serialize, Deserialize)]
pub struct HttpCallbackAckPayload {
    pub id: Option<String>,
    pub s: Option<u64>,
    pub op: HttpCallbackAckOp,
}

/// opcode为13时，开放平台对机器人服务端进行验证时的消息对象
#[derive(Debug, Serialize, Deserialize)]
pub struct WebhookAddressVerifyPayload {
    pub id: Option<String>,
    pub s: Option<u64>,
    pub op: WebhookAddressVerifyOp,
    pub d: ValidationRequest,
}
