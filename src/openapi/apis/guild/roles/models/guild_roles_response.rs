use serde::{Deserialize, Serialize};

use super::role::Role;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuildRolesResponse {
    pub guild_id: String,
    pub roles: Vec<Role>,
    pub role_num_limit: String,
}
