use std::sync::Arc;

use super::super::api::QQApiClient;
use super::super::api_request::ApiRequest;

pub mod create_direct_message;
mod me;
pub mod me_guilds;

pub struct UserApis {
    pub(crate) request: Arc<dyn ApiRequest>,
}

impl QQApiClient {
    pub fn user(&self) -> UserApis {
        UserApis {
            request: Arc::clone(&self.request),
        }
    }
}
