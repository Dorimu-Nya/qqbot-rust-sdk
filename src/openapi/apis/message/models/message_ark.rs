use serde::{Deserialize, Serialize};

use super::message_ark_kv::MessageArkKv;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageArk {
    pub template_id: u64,
    pub kv: Vec<MessageArkKv>,
}
