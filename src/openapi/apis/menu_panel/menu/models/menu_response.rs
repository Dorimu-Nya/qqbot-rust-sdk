use serde::{Deserialize, Serialize};

use super::menu::Menu;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MenuResponse {
    pub version: i64,
    #[serde(default)]
    pub menu: Option<Menu>,
}
