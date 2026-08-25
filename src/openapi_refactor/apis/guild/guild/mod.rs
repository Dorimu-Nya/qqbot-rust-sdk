use std::sync::Arc;

use super::super::super::api_request::ApiRequest;
use super::GuildApis;

mod guild;

pub struct GuildApi {
    pub(crate) request: Arc<dyn ApiRequest>,
}

impl GuildApis {
    pub fn guild(&self) -> GuildApi {
        GuildApi {
            request: Arc::clone(&self.request),
        }
    }
}
