use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadMediaRequest {
    pub file_type: u8,
    pub url: String,
    pub srv_send_msg: bool,
    #[serde(default)]
    pub file_data: Option<String>,
}
