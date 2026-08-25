use std::sync::Arc;

use super::super::super::api_request::ApiRequest;
use super::GuildApis;

mod guild_mute;
mod member_mute;
pub mod models;
mod multi_member_mute;

pub struct MuteApi {
    pub(crate) request: Arc<dyn ApiRequest>,
}

impl GuildApis {
    pub fn mute(&self) -> MuteApi {
        MuteApi {
            request: Arc::clone(&self.request),
        }
    }
}
