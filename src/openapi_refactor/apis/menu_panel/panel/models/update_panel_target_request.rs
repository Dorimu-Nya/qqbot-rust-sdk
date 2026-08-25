use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePanelTargetRequest {
    pub op: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_openids: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group_openids: Option<Vec<String>>,
}
