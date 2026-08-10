use serde_repr::{Deserialize_repr, Serialize_repr};

/// 参考: <https://bot.q.qq.com/wiki/develop/api-v2/dev-prepare/event-emit/payload.html#opcode-%E5%90%AB%E4%B9%89>

/// 服务端进行消息推送
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)]
#[repr(u16)]
pub enum DispatchOp {
    Dispatch = 0,
}

/// 客户端或服务端发送心跳
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)]
#[repr(u16)]
pub enum Heartbeat {
    Heartbeat = 1,
}

/// 客户端发送鉴权
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)]
#[repr(u16)]
pub enum Identify {
    Identify = 2,
}

/// 客户端恢复连接
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)]
#[repr(u16)]
pub enum Resume {
    Resume = 6,
}

/// 服务端通知客户端重新连接
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)]
#[repr(u16)]
pub enum Reconnect {
    Reconnect = 7,
}

/// 当 identify 或 resume 的时候，如果参数有错，服务端会返回该消息
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)]
#[repr(u16)]
pub enum InvalidSession {
    InvalidSession = 9,
}

/// 当客户端与网关建立 ws 连接之后，网关下发的第一条消息
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)]
#[repr(u16)]
pub enum Hello {
    Hello = 10,
}

/// 当发送心跳成功之后，就会收到该消息
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)]
#[repr(u16)]
pub enum HeartbeatACK {
    HeartbeatACK = 11,
}

/// 仅用于 http 回调模式的回包，代表机器人收到了平台推送的数据
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)]
#[repr(u16)]
pub enum HttpCallbackAckOp {
    HttpCallbackAck = 12,
}

/// 开放平台对机器人服务端进行验证
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)]
#[repr(u16)]
pub enum WebhookAddressVerifyOp {
    WebhookAddressVerify = 13,
}
