use std::sync::Arc;

use super::super::super::api_request::ApiRequest;
use super::MessageApis;

mod post_direct_message;
mod retract_dm_message;

pub struct DirectApi {
    pub(crate) request: Arc<dyn ApiRequest>,
}

impl MessageApis {
    pub fn direct(&self) -> DirectApi {
        DirectApi {
            request: Arc::clone(&self.request),
        }
    }
}
