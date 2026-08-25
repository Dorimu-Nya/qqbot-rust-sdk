use serde::{Deserialize, Serialize};

use super::KeyboardRow;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyboardContent {
    pub rows: Vec<KeyboardRow>,
}
