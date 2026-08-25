use serde::{Deserialize, Serialize};

use super::sub_menu_item::SubMenuItem;
use super::switch::Switch;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MenuItem {
    pub name: String,
    #[serde(rename = "type")]
    pub kind: String,
    #[serde(default)]
    pub sub_menu_items: Vec<SubMenuItem>,
    #[serde(default)]
    pub send_message: Option<String>,
    #[serde(default)]
    pub link: Option<String>,
    #[serde(default)]
    pub r#switch: Option<Switch>,
}
