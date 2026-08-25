use serde::{Deserialize, Serialize};

use super::api_permission_demand_identify::ApiPermissionDemandIdentify;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateApiPermissionDemandRequest {
    pub channel_id: String,
    pub api_identify: ApiPermissionDemandIdentify,
    pub desc: String,
}
