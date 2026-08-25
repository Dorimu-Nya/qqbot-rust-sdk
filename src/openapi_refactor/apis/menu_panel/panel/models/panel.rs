use serde::{Deserialize, Serialize};

use super::PanelItem;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Panel {
    #[serde(default)]
    pub items: Vec<PanelItem>,
    #[serde(default)]
    pub remark: Option<String>,
    #[serde(default)]
    pub version: Option<i64>,
}
