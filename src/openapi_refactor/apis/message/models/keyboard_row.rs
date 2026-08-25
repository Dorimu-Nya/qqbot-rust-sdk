use serde::{Deserialize, Serialize};

use super::KeyboardButton;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyboardRow {
    pub buttons: Vec<KeyboardButton>,
}
