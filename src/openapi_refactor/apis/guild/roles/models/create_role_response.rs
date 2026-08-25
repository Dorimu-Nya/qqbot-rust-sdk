use serde::{Deserialize, Serialize};

use super::role::Role;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRoleResponse {
    pub role_id: String,
    pub role: Role,
}
