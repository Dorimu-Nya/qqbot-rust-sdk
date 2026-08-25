use std::sync::Arc;

use super::super::super::api_request::ApiRequest;
use super::MessageApis;

pub mod models;
mod upload_c2c_media;
mod upload_group_media;

pub struct MediaApi {
    pub(crate) request: Arc<dyn ApiRequest>,
}

impl MessageApis {
    pub fn media(&self) -> MediaApi {
        MediaApi {
            request: Arc::clone(&self.request),
        }
    }
}
