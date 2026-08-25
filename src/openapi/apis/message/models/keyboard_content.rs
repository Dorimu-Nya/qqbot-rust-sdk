use serde::{Deserialize, Serialize};

use super::keyboard_row::KeyboardRow;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyboardContent {
    pub rows: Vec<KeyboardRow>,
}
