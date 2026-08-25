use serde::{Deserialize, Serialize};

use super::api_permission::ApiPermission;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiPermissionsResponse {
    pub apis: Vec<ApiPermission>,
}
