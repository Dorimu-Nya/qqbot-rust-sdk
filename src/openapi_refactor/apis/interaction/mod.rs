use std::sync::Arc;

use super::super::api::QQApiClient;
use super::super::api_request::ApiRequest;

pub mod put_interaction;

pub struct InteractionApis {
    pub(crate) request: Arc<dyn ApiRequest>,
}

impl QQApiClient {
    pub fn interaction(&self) -> InteractionApis {
        InteractionApis {
            request: Arc::clone(&self.request),
        }
    }
}
