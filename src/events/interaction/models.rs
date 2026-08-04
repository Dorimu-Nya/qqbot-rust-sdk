use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
/// 解析后的互动数据
///
/// 参考: <https://bot.q.qq.com/wiki/develop/api-v2/autogen/event/interaction_create.html#schema-interactionresolved>
pub struct InteractionResolved {
    /// 操作按钮的 data 字段值（在发送消息按钮时设置）
    pub button_data: Option<String>,
    /// 操作按钮的 id 字段值（在发送消息按钮时设置）
    pub button_id: Option<String>,
    /// 操作的用户 userid，仅频道场景提供该字段
    pub user_id: Option<String>,
    /// 操作按钮的 feature_id，仅快捷菜单提供该字段（在管理端设置）
    pub feature_id: Option<String>,
    /// 操作的消息 id（频道场景为消息 OpenID；消息反馈场景为机器人消息 ID）
    pub message_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// 互动数据
///
/// 参考: <https://bot.q.qq.com/wiki/develop/api-v2/autogen/event/interaction_create.html#schema-interactiondata>
pub struct InteractionData {
    #[serde(default, alias = "resoloved")]
    /// 解析后的互动数据
    pub resolved: Option<InteractionResolved>,
}

/// 互动事件 事件体
///
/// 参考: <https://bot.q.qq.com/wiki/develop/api-v2/autogen/event/interaction_create.html#%E4%BA%8B%E4%BB%B6%E4%BD%93>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Interaction {
    /// 平台方事件 ID，可以用于被动消息发送
    pub id: String,
    #[serde(rename = "type")]
    /// 消息类型： 11表示消息按钮; 12表示单聊快捷菜单; 13表示消息反馈; 14表示清空会话
    ///
    /// 15表示进出故事集; 16表示切换智能体模型; 18表示授权; 19表示群授权; 20表示群授权状态变更
    pub kind: Option<i64>,
    /// 事件发生的场景：c2c、group、guild
    pub scene: Option<String>,
    /// 聊天类型：0 频道场景，1 群聊场景，2 单聊场景
    pub chat_type: Option<i64>,
    /// 触发时间，RFC 3339 格式
    pub timestamp: Option<String>,
    /// 频道的 openid，仅在频道场景提供该字段
    pub guild_id: Option<String>,
    /// 子频道的 openid，仅在频道场景提供该字段
    pub channel_id: Option<String>,
    /// 用户 openid，仅在单聊场景提供该字段
    pub user_openid: Option<String>,
    /// 群 openid，仅在群聊场景提供该字段
    pub group_openid: Option<String>,
    /// 群成员 openid，仅在群聊场景提供该字段
    pub group_member_openid: Option<String>,
    /// 互动数据
    pub data: Option<InteractionData>,
    /// 版本号，默认 1
    pub version: Option<i64>,
}
