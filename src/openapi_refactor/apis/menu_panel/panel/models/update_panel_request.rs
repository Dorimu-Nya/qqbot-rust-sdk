use serde::{Deserialize, Serialize};

use super::Panel;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePanelRequest {
    pub panel: Panel,
}
