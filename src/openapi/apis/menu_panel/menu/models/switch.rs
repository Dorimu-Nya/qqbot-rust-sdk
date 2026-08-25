use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Switch {
    pub switch_id: String,
    pub default: bool,
}
