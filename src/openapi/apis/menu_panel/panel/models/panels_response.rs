use serde::{Deserialize, Serialize};

use super::panel_record::PanelRecord;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PanelsResponse {
    #[serde(default)]
    pub records: Vec<PanelRecord>,
    #[serde(default)]
    pub next_cursor: String,
    #[serde(default)]
    pub is_end: bool,
}
