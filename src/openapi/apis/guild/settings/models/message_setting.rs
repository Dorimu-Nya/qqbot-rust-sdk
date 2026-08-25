use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageSetting {
    pub disable_create_dm: bool,
    pub disable_push_msg: bool,
    pub channel_ids: Vec<String>,
    pub channel_push_max_num: u32,
}
