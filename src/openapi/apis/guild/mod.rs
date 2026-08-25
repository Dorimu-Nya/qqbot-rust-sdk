use std::sync::Arc;

use super::super::api::QQApiClient;
use super::super::api_request::ApiRequest;

pub mod announces;
pub mod channel;
pub mod guild;
pub mod members;
pub mod mute;
pub mod permissions;
pub mod roles;
pub mod settings;

pub struct GuildApis {
    pub(crate) request: Arc<dyn ApiRequest>,
}

impl QQApiClient {
    pub fn guild(&self) -> GuildApis {
        GuildApis {
            request: Arc::clone(&self.request),
        }
    }
}
