use std::sync::Arc;

use super::super::super::api_request::ApiRequest;
use super::MessageApis;

mod post_c2c_message;
mod retract_c2c_message;

pub struct C2cApi {
    pub(crate) request: Arc<dyn ApiRequest>,
}

impl MessageApis {
    pub fn c2c(&self) -> C2cApi {
        C2cApi {
            request: Arc::clone(&self.request),
        }
    }
}
