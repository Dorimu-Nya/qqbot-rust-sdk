//! 基于 [QQ官方机器人文档](https://bot.q.qq.com/wiki/develop/api-v2/) 中声明的事件数据模型、API及相关所需要的工具类所开发的QQ官方机器人，仅提供了事件类型、数据模型、QQ官方API的定义等。
//
// 目前是围绕着Webhook的需要去开发的，关于在Websocket连接方式仅有的一些数据可能有缺失，后面会慢慢补上。
//
// 根据自己的需要添加对应的Feature，默认则是全部。

#[cfg(feature = "events")]
pub mod events;

#[cfg(feature = "openapi")]
pub mod openapi;

#[cfg(feature = "signature")]
pub mod signature;
