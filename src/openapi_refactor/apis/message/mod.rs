use std::sync::Arc;

use super::super::api::QQApiClient;
use super::super::api_request::ApiRequest;

pub mod c2c;
pub mod direct;
pub mod group;
pub mod media;
pub mod models;

pub struct MessageApis {
    pub(crate) request: Arc<dyn ApiRequest>,
}

impl QQApiClient {
    pub fn message(&self) -> MessageApis {
        MessageApis {
            request: Arc::clone(&self.request),
        }
    }
}
