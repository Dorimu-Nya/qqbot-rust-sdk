use std::sync::Arc;

use super::api_request::ApiRequest;

pub struct QQApiClient {
    pub(crate) request: Arc<dyn ApiRequest>,
}
