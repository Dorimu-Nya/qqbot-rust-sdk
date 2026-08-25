use serde::{Deserialize, Serialize};

use super::Role;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateRoleResponse {
    pub guild_id: String,
    pub role_id: String,
    pub role: Role,
}
