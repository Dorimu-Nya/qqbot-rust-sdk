use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
/// 表情对象
///
/// 参考: <https://bot.q.qq.com/wiki/develop/api-v2/openapi/emoji/model.html#Emoji>
pub struct Emoji {
    pub id: String,
    #[serde(rename = "type")]
    pub emoji_type: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// 表态对象
///
/// 参考: <https://bot.q.qq.com/wiki/develop/api-v2/openapi/reaction/model.html#reactiontarget>
pub struct ReactionTarget {
    #[serde(rename = "type")]
    /// 表态对象类型: 0消息; 1帖子; 2评论; 3回复
    ///
    /// 参考: <https://bot.q.qq.com/wiki/develop/api-v2/openapi/reaction/model.html#reactiontargettype>
    pub target_type: Option<i64>,
    /// 表态对象ID
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// 表情表态对象
///
/// 参考: <https://bot.q.qq.com/wiki/develop/api-v2/openapi/reaction/model.html#messagereaction>
pub struct MessageReaction {
    /// 用户ID
    pub user_id: String,
    /// 频道ID
    pub guild_id: String,
    /// 子频道ID
    pub channel_id: String,
    /// 表态对象
    pub target: ReactionTarget,
    /// 表态所用表情
    pub emoji: Emoji,
}
