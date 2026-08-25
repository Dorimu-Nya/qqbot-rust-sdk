use std::sync::Arc;

use super::super::super::api_request::ApiRequest;
use super::GuildApis;

mod channels;
pub mod models;
mod post_channel;

pub struct ChannelApi {
    pub(crate) request: Arc<dyn ApiRequest>,
}

impl GuildApis {
    pub fn channel(&self) -> ChannelApi {
        ChannelApi {
            request: Arc::clone(&self.request),
        }
    }
}
