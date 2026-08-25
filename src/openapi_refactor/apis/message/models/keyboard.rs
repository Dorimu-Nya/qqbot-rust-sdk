use serde::{Deserialize, Serialize};

use super::KeyboardContent;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Keyboard {
    pub content: KeyboardContent,
}
