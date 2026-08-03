//! 通用数据结构定义，包括Payload结构体，Opcode定义，以及事件的分类
//!
//! 参考： <https://bot.q.qq.com/wiki/develop/api-v2/dev-prepare/event-emit/payload.html>

/// 汇总事件类型的分类
///
/// 参考1: <https://bot.q.qq.com/wiki/develop/api-v2/dev-prepare/event-emit/payload.html#%E4%BA%8B%E4%BB%B6%E8%AE%A2%E9%98%85-intents>
///
/// 参考2: <https://bot.q.qq.com/wiki/develop/gosdk/websocket/listen_events.html>
///
/// 参考3: <https://bot.q.qq.com/wiki/develop/nodesdk/wss/model.html>
pub mod event;

/// Opcode定义
///
/// 参考： <https://bot.q.qq.com/wiki/develop/api-v2/dev-prepare/event-emit/payload.html#opcode-%E5%90%AB%E4%B9%89>
pub mod opcode;

/// 根据Opcode结合Rust枚举风格的事件类型定义。
///
/// 参考： <https://bot.q.qq.com/wiki/develop/api-v2/dev-prepare/event-emit/payload.html#%E9%80%9A%E7%94%A8%E6%95%B0%E6%8D%AE%E7%BB%93%E6%9E%84>
pub mod payload;