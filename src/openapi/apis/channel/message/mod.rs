use std::sync::Arc;

use super::super::super::api_request::ApiRequest;
use super::ChannelApis;

mod message;
pub mod models;
mod patch_message;
mod post_message;
mod retract_message;

pub struct MessageApi {
    pub(crate) request: Arc<dyn ApiRequest>,
}

impl ChannelApis {
    pub fn message(&self) -> MessageApi {
        MessageApi {
            request: Arc::clone(&self.request),
        }
    }
}
