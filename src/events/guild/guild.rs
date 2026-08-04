use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
/// 频道
///
/// 参考1: <https://bot.q.qq.com/wiki/develop/api-v2/autogen/event/guild_create.html>
///
/// 参考2: <https://bot.q.qq.com/wiki/develop/api-v2/autogen/event/guild_update.html>
///
/// 参考3: <https://bot.q.qq.com/wiki/develop/api-v2/autogen/event/guild_delete.html>
pub struct GuildData {
    /// 频道ID
    pub id: String,
    /// 频道名称
    pub name: Option<String>,
    /// 频道头像地址
    pub icon: Option<String>,
    /// 创建人用户ID
    pub owner_id: Option<String>,
    /// 成员数
    pub member_count: Option<i64>,
    /// 最大成员数
    pub max_members: Option<i64>,
    /// 描述
    pub description: Option<String>,
    /// 加入时间
    pub joined_at: Option<String>,
    /// 操作人用户ID
    pub op_user_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// 子频道
///
/// 参考1: <https://bot.q.qq.com/wiki/develop/api-v2/autogen/event/channel_create.html>
///
/// 参考2: <https://bot.q.qq.com/wiki/develop/api-v2/autogen/event/channel_update.html>
///
/// 参考3: <https://bot.q.qq.com/wiki/develop/api-v2/autogen/event/channel_delete.html>
pub struct ChannelData {
    /// 子频道ID
    pub id: String,
    /// 频道ID
    pub guild_id: String,
    /// 子频道名
    pub name: Option<String>,
    #[serde(rename = "type")]
    /// 子频道类型: 0=文字, 2=语音, 4=分组, 10005=直播, 10006=应用, 10007=论坛
    pub channel_type: Option<i64>,
    /// 子频道子类型（文字子频道）: 0=闲聊, 1=公告, 2=攻略, 3=开黑
    pub sub_type: Option<i64>,
    /// 创建人ID
    pub owner_id: Option<String>,
    /// 操作人用户ID
    pub op_user_id: Option<String>,
}
