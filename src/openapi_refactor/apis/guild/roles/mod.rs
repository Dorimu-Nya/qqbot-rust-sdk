use std::sync::Arc;

use super::super::super::api_request::ApiRequest;
use super::GuildApis;

mod delete_role;
mod guild_role_members;
mod member_add_role;
mod member_delete_role;
pub mod models;
mod patch_role;
mod post_role;
mod roles;

pub struct RolesApi {
    pub(crate) request: Arc<dyn ApiRequest>,
}

impl GuildApis {
    pub fn roles(&self) -> RolesApi {
        RolesApi {
            request: Arc::clone(&self.request),
        }
    }
}
