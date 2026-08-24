use serde::{Deserialize, Serialize};

/// GET /v2/menu 返回参数。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MenuResponse {
    pub version: i64,
    #[serde(default)]
    pub menu: Option<Menu>,
}

/// PUT /v2/menu 请求参数。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MenuPutRequest {
    /// 菜单配置；传入后会覆盖原有的完整菜单配置。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub menu: Option<Menu>,
}

/// PUT /v2/menu 返回参数。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MenuPutResponse {
    /// 本次修改后的菜单版本号。
    pub version: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Menu {
    #[serde(default)]
    pub items: Vec<MenuItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MenuItem {
    pub name: String,
    #[serde(rename = "type")]
    pub kind: String,
    #[serde(default)]
    pub sub_menu_items: Vec<SubMenuItem>,
    #[serde(default)]
    pub send_message: Option<String>,
    #[serde(default)]
    pub link: Option<String>,
    #[serde(default)]
    pub r#switch: Option<Switch>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubMenuItem {
    pub name: String,
    #[serde(rename = "type")]
    pub kind: String,
    #[serde(default)]
    pub send_message: Option<String>,
    #[serde(default)]
    pub link: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Switch {
    pub switch_id: String,
    pub default: bool,
}

/// GET /v2/panels 请求参数。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PanelsQuery {
    /// 生效场景：`c2c`、`group`、`channel` 或 `dm`。
    pub scope: String,
    /// 分页游标。
    #[serde(default)]
    pub cursor: Option<String>,
    /// 每页拉取条数，默认 20，最大 50。
    #[serde(default)]
    pub limit: Option<u32>,
}

/// `PanelsQuery` 的兼容性别名。
pub type PanelListQuery = PanelsQuery;

/// GET /v2/panels 返回参数。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PanelsResponse {
    #[serde(default)]
    pub records: Vec<PanelRecord>,
    #[serde(default)]
    pub next_cursor: String,
    #[serde(default)]
    pub is_end: bool,
}

/// POST /v2/panels 请求参数。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePanelRequest {
    /// 生效场景：`c2c`、`group`、`channel` 或 `dm`。
    pub scope: String,
    /// 作用范围：`all` 或 `specific`。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_type: Option<String>,
    /// `c2c` 且 `target_type=specific` 时生效的用户 openid 列表。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_openids: Option<Vec<String>>,
    /// `group` 且 `target_type=specific` 时生效的群 openid 列表。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group_openids: Option<Vec<String>>,
    pub panel: Panel,
}

/// POST /v2/panels 返回参数。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePanelResponse {
    pub panel_id: String,
}

/// 指令面板记录。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PanelRecord {
    pub panel_id: String,
    pub scope: String,
    pub target_type: String,
    pub panel: Panel,
    #[serde(default)]
    pub created_at: Option<String>,
    #[serde(default)]
    pub updated_at: Option<String>,
    pub version: i64,
    /// 关联的用户 openid 列表，仅 C2C 指定用户面板返回。
    #[serde(default)]
    pub user_openids: Vec<String>,
    /// 关联的群 openid 列表，仅群聊指定群面板返回。
    #[serde(default)]
    pub group_openids: Vec<String>,
}

/// GET /v2/panels/{panel_id} 返回参数。
pub type PanelDetailResponse = PanelRecord;

/// PUT /v2/panels/{panel_id} 请求参数。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePanelRequest {
    /// 面板配置；传入后会覆盖原有的面板元素列表和备注。
    pub panel: Panel,
}

/// PUT /v2/panels/{panel_id} 返回参数。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePanelResponse {
    /// 本次修改后的面板版本号。
    pub version: i64,
}

/// PUT /v2/panels/{panel_id}/target 请求参数。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePanelTargetRequest {
    /// 操作类型：`add` 添加关联对象，`del` 移除关联对象。
    pub op: String,
    /// C2C 场景下要操作的用户 openid 列表。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_openids: Option<Vec<String>>,
    /// 群聊场景下要操作的群 openid 列表。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group_openids: Option<Vec<String>>,
}

/// 指令面板配置。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Panel {
    #[serde(default)]
    pub items: Vec<PanelItem>,
    #[serde(default)]
    pub remark: Option<String>,
    #[serde(default)]
    pub version: Option<i64>,
}

/// 指令面板元素。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PanelItem {
    pub name: String,
    pub desc: String,
    #[serde(rename = "type")]
    pub kind: String,
    #[serde(default)]
    pub only_admin: bool,
    #[serde(default)]
    pub link: Option<String>,
}
