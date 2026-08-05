//! 在各种事件类型通用的数据结构定义

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// User
///
/// 参考1: <https://bot.q.qq.com/wiki/develop/api-v2/autogen/event/c2c_message_create.html#schema-user>
///
/// 参考2: <https://bot.q.qq.com/wiki/develop/api-v2/autogen/event/group_at_message_create.html#schema-user>
///
/// 参考3: <https://bot.q.qq.com/wiki/develop/api-v2/autogen/event/group_message_create.html#schema-user>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    /// 用户唯一标识（OpenID 格式）
    pub id: Option<String>,
    /// 用户昵称
    pub username: String,
    /// 是否为机器人
    pub bot: bool,
    /// 跨应用统一用户 OpenID（可能为空）
    pub union_openid: Option<String>,
    /// 跨应用统一用户账号（可能为空）
    pub union_user_account: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// 单聊场景用户
pub struct C2cUser {
    /// 通用用户信息
    #[serde(flatten)]
    pub user: User,
    /// 用户 OpenID（单聊场景使用）
    pub user_openid: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// 群聊场景用户
pub struct GroupUser {
    /// 通用用户信息
    #[serde(flatten)]
    pub user: User,
    /// 群成员 OpenID（群聊场景使用）
    pub member_openid: String,
    /// 群内角色。member=普通成员, admin=管理员, owner=群主
    pub member_role: String,
}

/// 消息场景
///
/// 参考1: <https://bot.q.qq.com/wiki/develop/api-v2/autogen/event/c2c_message_create.html#schema-messagescene>
///
/// 参考2: <https://bot.q.qq.com/wiki/develop/api-v2/autogen/event/group_at_message_create.html#schema-arkdata>
///
/// 参考3: <https://bot.q.qq.com/wiki/develop/api-v2/autogen/event/group_message_create.html#schema-messagescene>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageScene {
    /// 场景来源。default=默认聊天窗口
    pub source: Option<String>,
    /// 扩展数据列表，key=value 格式: msg_idx=消息索引, 用于引用场景 ref_msg_idx=引用的消息索引 auth_token=鉴权令牌
    pub ext: Option<Vec<String>>,
}

/// 消息附件
///
/// 参考1: <https://bot.q.qq.com/wiki/develop/api-v2/autogen/event/c2c_message_create.htmll#schema-messageattachment>
///
/// 参考2: <https://bot.q.qq.com/wiki/develop/api-v2/autogen/event/group_at_message_create.htmll#schema-messageattachment>
///
/// 参考3: <https://bot.q.qq.com/wiki/develop/api-v2/autogen/event/group_message_create.html#schema-messageattachment>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageAttachment {
    pub content_type: Option<String>,
    pub filename: Option<String>,
    pub height: Option<i64>,
    pub width: Option<i64>,
    pub size: Option<i64>,
    pub url: Option<String>,
}

/// ARKData
///
/// 参考1: <https://bot.q.qq.com/wiki/develop/api-v2/autogen/event/c2c_message_create.html#schema-arkdata>
///
/// 参考2: <https://bot.q.qq.com/wiki/develop/api-v2/autogen/event/group_at_message_create.html#schema-arkdata>
///
/// 参考3: <https://bot.q.qq.com/wiki/develop/api-v2/autogen/event/group_message_create.html#schema-arkdata>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ARKData {
    /// 卡片消息中的用户操作提示文本
    pub prompt: String,
    /// 卡片消息类型标识
    ///
    /// 如：tuwen = 图文 H5（如快手分享链接） feed = 图文卡片（群相册、频道帖子、分享卡片） miniapp = 小程序（微信小程序、QQ 小程序、哔哩哔哩等） map = 位置卡片 contact_card = 好友名片 video_share = 视频分享 music_together = 一起听歌
    pub ark_type: String,
    /// 卡片消息类型的中文名称，如"图文 H5"、"小程序"、"图文卡片"
    pub ark_name: String,
    /// 卡片消息字段
    ///
    /// 常见键名: tag/tags=来源标签, title=标题, desc=描述, jump_url=跳转链接, preview=预览图, source=来源名称, source_logo=来源图标, tag_icon=标签图标, nickname=昵称, avatar=头像, address=地址
    pub fields: HashMap<String, String>,
}

/// MsgElement
///
/// 参考1: <https://bot.q.qq.com/wiki/develop/api-v2/autogen/event/c2c_message_create.html#schema-msgelement>
///
/// 参考2: <https://bot.q.qq.com/wiki/develop/api-v2/autogen/event/group_at_message_create.html#schema-msgelement>
///
/// 参考3: <https://bot.q.qq.com/wiki/develop/api-v2/autogen/event/group_message_create.html#schema-msgelement>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MsgElement {
    /// 消息元素在列表中的引用消息索引
    pub msg_idx: Option<String>,
    /// 该元素对应的消息发送者
    pub author: Option<User>,
    /// 消息内容类型: 0=普通文本, 3=结构化卡片, 101=并行消息, 102=聊天记录, 103=引用消息
    pub message_type: Option<i32>,
    /// 消息正文内容
    pub content: Option<String>,
    /// 该元素携带的附件
    pub attachments: Option<Vec<MessageAttachment>>,
    /// 结构化卡片消息数据（message_type=3 时有值）
    pub ark_data: Option<ARKData>,
    /// 嵌套消息元素列表（递归结构）
    pub msg_elements: Option<Vec<MsgElement>>,
}
