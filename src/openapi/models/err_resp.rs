use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrResp {
    pub err_code: u64,
    pub message: String,
    pub trace_id: String,
}
