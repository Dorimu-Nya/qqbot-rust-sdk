use serde::{Deserialize, Serialize};

use super::ButtonStyle;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenderData {
    pub label: String,
    pub visited_label: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<ButtonStyle>,
}
