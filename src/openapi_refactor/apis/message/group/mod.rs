use std::sync::Arc;

use super::super::super::api_request::ApiRequest;
use super::MessageApis;

mod get_group_members;
pub mod models;
mod post_group_message;
mod retract_group_message;

pub struct GroupApi {
    pub(crate) request: Arc<dyn ApiRequest>,
}

impl MessageApis {
    pub fn group(&self) -> GroupApi {
        GroupApi {
            request: Arc::clone(&self.request),
        }
    }
}
