use std::sync::Arc;

use super::super::super::api_request::ApiRequest;
use super::ChannelApis;

mod add_pins;
mod delete_pins;
mod get_pins;
pub mod models;

pub struct PinsApi {
    pub(crate) request: Arc<dyn ApiRequest>,
}

impl ChannelApis {
    pub fn pins(&self) -> PinsApi {
        PinsApi {
            request: Arc::clone(&self.request),
        }
    }
}
