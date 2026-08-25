use serde::{Deserialize, Serialize};

use super::api_permission_demand_identify::ApiPermissionDemandIdentify;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiPermissionDemand {
    pub guild_id: String,
    pub channel_id: String,
    pub api_identify: ApiPermissionDemandIdentify,
    pub title: String,
    pub desc: String,
}
