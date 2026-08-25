use serde::{Deserialize, Serialize};

use super::Menu;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MenuPutRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub menu: Option<Menu>,
}
