use std::sync::Arc;

use super::super::super::api_request::ApiRequest;
use super::ChannelApis;

mod delete_mic;
mod list_voice_channel_members;
pub mod post_audio;
mod put_mic;

pub struct AudioApi {
    pub(crate) request: Arc<dyn ApiRequest>,
}

impl ChannelApis {
    pub fn audio(&self) -> AudioApi {
        AudioApi {
            request: Arc::clone(&self.request),
        }
    }
}
