use serde::{Deserialize, Serialize};

use super::{Action, RenderData};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyboardButton {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub render_data: RenderData,
    pub action: Action,
}
