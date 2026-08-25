use serde::{Deserialize, Serialize};

use super::ApiPermission;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiPermissionsResponse {
    pub apis: Vec<ApiPermission>,
}
