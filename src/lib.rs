//! 基于 [QQ官方机器人文档](https://bot.q.qq.com/wiki/develop/api-v2/) 中声明的事件数据模型、API接口封装及相关所需要的工具类所开发的QQ官方机器人SDK。
//!
//! 仅提供了事件类型、数据模型、QQ官方API的定义等。
//!
//! 目前是围绕着Webhook的需要去开发的，关于在Websocket连接方式仅有的一些数据可能有缺失，后面会慢慢补上。
//!
//! 根据自己的需要添加对应的Feature，默认则是全部。

#[cfg(feature = "events")]
/// 存放来自Webhook回调的事件类型及模型字段定义
pub mod events;

#[cfg(feature = "openapi")]
/// 提供根据文档提供的QQ机器人相关的OpenApi接口。
///
/// 参考： <https://bot.q.qq.com/wiki/develop/api-v2/autogen/>
pub mod openapi;

mod openapi_refactor;
#[cfg(feature = "signature")]
/// 提供根据文档开发的签名校验逻辑。
///
/// 参考： <https://bot.q.qq.com/wiki/develop/api-v2/dev-prepare/interface-framework/sign.html>
pub mod signature;
