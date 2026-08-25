use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Thread {
    #[serde(default, alias = "id", alias = "thread_id")]
    pub thread_id: Option<String>,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub content: Option<Value>,
    #[serde(default)]
    pub date_time: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}
