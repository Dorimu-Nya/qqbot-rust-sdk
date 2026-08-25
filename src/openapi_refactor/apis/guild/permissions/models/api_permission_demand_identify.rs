use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiPermissionDemandIdentify {
    pub path: String,
    pub method: String,
}
