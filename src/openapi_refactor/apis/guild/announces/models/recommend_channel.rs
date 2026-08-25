use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RecommendChannel {
    #[serde(default)]
    pub channel_id: Option<String>,
    #[serde(default)]
    pub introduce: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}
