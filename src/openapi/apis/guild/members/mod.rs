use std::sync::Arc;

use super::super::super::api_request::ApiRequest;
use super::GuildApis;

mod delete_guild_member;
mod guild_member;
mod guild_members;
pub mod models;

pub struct MembersApi {
    pub(crate) request: Arc<dyn ApiRequest>,
}

impl GuildApis {
    pub fn members(&self) -> MembersApi {
        MembersApi {
            request: Arc::clone(&self.request),
        }
    }
}
