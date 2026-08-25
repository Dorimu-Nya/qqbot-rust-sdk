use serde::{Deserialize, Serialize};

use super::PermissionType;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Permission {
    #[serde(rename = "type")]
    pub permission_type: PermissionType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub specify_user_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub specify_role_ids: Option<Vec<String>>,
}
