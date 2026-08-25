use std::sync::Arc;

use super::super::super::api_request::ApiRequest;
use super::GuildApis;

mod get_message_setting;
pub mod models;

pub struct SettingsApi {
    pub(crate) request: Arc<dyn ApiRequest>,
}

impl GuildApis {
    pub fn settings(&self) -> SettingsApi {
        SettingsApi {
            request: Arc::clone(&self.request),
        }
    }
}
