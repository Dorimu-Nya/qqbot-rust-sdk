use serde::{Deserialize, Serialize};

use super::menu_item::MenuItem;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Menu {
    #[serde(default)]
    pub items: Vec<MenuItem>,
}
