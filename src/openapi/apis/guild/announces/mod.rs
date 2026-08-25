use std::sync::Arc;

use super::super::super::api_request::ApiRequest;
use super::GuildApis;

mod clean_channel_announces;
mod clean_guild_announces;
mod create_channel_announces;
mod create_guild_announces;
mod delete_channel_announces;
mod delete_guild_announces;
pub mod models;

pub struct AnnouncesApi {
    pub(crate) request: Arc<dyn ApiRequest>,
}

impl GuildApis {
    pub fn announces(&self) -> AnnouncesApi {
        AnnouncesApi {
            request: Arc::clone(&self.request),
        }
    }
}
