use std::sync::Arc;

use super::super::super::api_request::ApiRequest;
use super::ChannelApis;

mod create_thread;
mod delete_thread;
mod get_thread;
mod list_threads;
pub mod models;

pub struct ThreadApi {
    pub(crate) request: Arc<dyn ApiRequest>,
}

impl ChannelApis {
    pub fn thread(&self) -> ThreadApi {
        ThreadApi {
            request: Arc::clone(&self.request),
        }
    }
}
