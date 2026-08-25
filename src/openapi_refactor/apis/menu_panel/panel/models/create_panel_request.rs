use serde::{Deserialize, Serialize};

use super::Panel;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePanelRequest {
    pub scope: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_openids: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group_openids: Option<Vec<String>>,
    pub panel: Panel,
}
