use serde::{Deserialize, Serialize};

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
