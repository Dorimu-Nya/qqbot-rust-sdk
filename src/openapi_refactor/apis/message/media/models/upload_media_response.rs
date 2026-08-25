use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UploadMediaResponse {
    #[serde(default)]
    pub file_uuid: Option<String>,
    #[serde(default)]
    pub file_info: Option<String>,
    #[serde(default)]
    pub ttl: Option<u64>,
    #[serde(default)]
    pub id: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}
