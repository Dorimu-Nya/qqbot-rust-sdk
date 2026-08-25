use serde::{Deserialize, Serialize};

use super::keyboard_content::KeyboardContent;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Keyboard {
    pub content: KeyboardContent,
}
