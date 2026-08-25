use serde::{Deserialize, Serialize};

use super::action::Action;
use super::render_data::RenderData;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyboardButton {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub render_data: RenderData,
    pub action: Action,
}
