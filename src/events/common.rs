//! 在各种事件类型通用的数据结构定义

use std::collections::HashMap;
use serde::{Deserialize, Serialize};

/// 附件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attachment {
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