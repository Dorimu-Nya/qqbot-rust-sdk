use serde::{Deserialize, Serialize};

use super::Panel;

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
    #[serde(default)]
    pub user_openids: Vec<String>,
    #[serde(default)]
    pub group_openids: Vec<String>,
}
