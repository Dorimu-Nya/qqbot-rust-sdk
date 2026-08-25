use std::sync::Arc;

use super::super::api::QQApiClient;
use super::super::api_request::ApiRequest;

pub mod audio;
pub mod channel;
pub mod message;
pub mod permissions;
pub mod pins;
pub mod reaction;
pub mod schedule;
pub mod thread;

pub struct ChannelApis {
    pub(crate) request: Arc<dyn ApiRequest>,
}

impl QQApiClient {
    pub fn channel(&self) -> ChannelApis {
        ChannelApis {
            request: Arc::clone(&self.request),
        }
    }
}
