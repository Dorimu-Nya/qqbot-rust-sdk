use super::super::common::MessageAttachment;
use super::member::{Member, User};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
/// 频道消息
/// 触发场景：频道内消息相关事件
///
/// 参考: <https://bot.q.qq.com/wiki/develop/api-v2/server-inter/message/template/model.html#message>
pub struct GuildMessages {
    /// 消息ID
    pub id: String,
    /// 子频道ID
    pub channel_id: Option<String>,
    /// 频道ID
    pub guild_id: Option<String>,
    /// 消息内容
    pub content: Option<String>,
    /// 消息创建时间
    pub timestamp: Option<String>,
    /// 消息编辑时间
    pub edited_timestamp: Option<String>,
    /// 是否是@全员消息
    pub mention_everyone: Option<bool>,
    /// 消息创建者
    pub author: Option<User>,
    /// 富媒体文件附件，文件类型："图片，语音，视频，文件"
    pub attachments: Option<Vec<MessageAttachment>>,
    /// embed
    pub embeds: Vec<MessageEmbed>,
    /// 消息中@的人
    pub mentions: Vec<User>,
    /// 消息创建者的member信息
    pub member: Option<Member>,
    /// ark消息
    pub ark: Option<MessageArk>,
    /// 消息序号
    ///
    /// 用于消息间的排序，seq 在同一子频道中按从先到后的顺序递增，不同的子频道之间消息无法排序。(目前只在消息事件中有值，2022年8月1日 后续废弃)
    #[deprecated]
    pub seq: Option<u64>,
    /// 子频道消息seq
    ///
    /// 子频道消息 seq，用于消息间的排序，seq 在同一子频道中按从先到后的顺序递增，不同的子频道之间消息无法排序
    pub seq_in_channel: Option<String>,
    /// 引用消息对象
    pub message_reference: Option<MessageReference>,
}
/// MessageEmbed
///
/// 参考: <https://bot.q.qq.com/wiki/develop/api-v2/server-inter/message/template/model.html#messageembed>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageEmbed {
    /// 标题
    pub title: Option<String>,
    /// 消息弹窗内容
    pub prompt: Option<String>,
    /// 缩略图
    pub thumbnail: Option<MessageEmbedThumbnail>,
    /// embed 字段数据
    pub fields: Option<Vec<MessageEmbedField>>,
}

/// MessageEmbedThumbnail
///
/// 参考: <https://bot.q.qq.com/wiki/develop/api-v2/server-inter/message/template/model.html#messageembedthumbnail>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageEmbedThumbnail {
    /// 图片地址
    pub url: Option<String>,
}

/// MessageEmbedField
///
/// 参考: <https://bot.q.qq.com/wiki/develop/api-v2/server-inter/message/template/model.html#messageembedfield>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageEmbedField {
    /// 字段名
    pub name: Option<String>,
}

/// MessageArk
///
/// 参考: <https://bot.q.qq.com/wiki/develop/api-v2/server-inter/message/template/model.html#messagearkkv>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageArk {
    /// ark模板id（需要先申请）
    pub template_id: u64,
    /// kv值列表
    pub kv: Vec<MessageArkKv>,
}

/// MessageArkKv
///
/// 参考: <https://bot.q.qq.com/wiki/develop/api-v2/server-inter/message/template/model.html#messagearkkv>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageArkKv {
    /// key
    pub key: String,
    /// value
    pub value: String,
    /// ark obj类型的列表
    pub obj: Option<Vec<MessageArkObj>>,
}

/// MessageArkObj
///
/// 参考: <https://bot.q.qq.com/wiki/develop/api-v2/server-inter/message/template/model.html#messagearkobj>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageArkObj {
    /// ark objkv列表
    pub obj_kv: Vec<MessageArkObjKv>,
}

/// MessageArkObjKv
///
/// 参考: <https://bot.q.qq.com/wiki/develop/api-v2/server-inter/message/template/model.html#messagearkobjkv>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageArkObjKv {
    /// key
    pub key: String,
    /// value
    pub value: String,
}

/// MessageReference
///
/// 参考: <https://bot.q.qq.com/wiki/develop/api-v2/server-inter/message/template/model.html#messagereference>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageReference {
    /// 需要引用回复的消息id
    pub message_id: String,
    /// 是否忽略获取引用消息详情错误，默认否
    pub ignore_get_message_error: Option<bool>,
}
