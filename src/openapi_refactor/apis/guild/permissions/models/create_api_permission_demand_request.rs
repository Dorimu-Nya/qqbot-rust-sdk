use serde::{Deserialize, Serialize};

use super::ApiPermissionDemandIdentify;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateApiPermissionDemandRequest {
    pub channel_id: String,
    pub api_identify: ApiPermissionDemandIdentify,
    pub desc: String,
}
