use std::sync::Arc;

use super::super::super::api_request::ApiRequest;
use super::ChannelApis;

mod channel;
mod delete_channel;
pub mod models;
mod online_nums;
mod patch_channel;

pub struct ChannelApi {
    pub(crate) request: Arc<dyn ApiRequest>,
}

impl ChannelApis {
    pub fn channel(&self) -> ChannelApi {
        ChannelApi {
            request: Arc::clone(&self.request),
        }
    }
}
