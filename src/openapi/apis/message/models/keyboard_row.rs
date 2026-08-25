use serde::{Deserialize, Serialize};

use super::keyboard_button::KeyboardButton;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyboardRow {
    pub buttons: Vec<KeyboardButton>,
}
