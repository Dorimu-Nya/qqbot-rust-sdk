use std::sync::Arc;

use super::super::super::api_request::ApiRequest;
use super::GuildApis;

mod get_api_permissions;
pub mod models;
mod require_api_permissions;

pub struct PermissionsApi {
    pub(crate) request: Arc<dyn ApiRequest>,
}

impl GuildApis {
    pub fn permissions(&self) -> PermissionsApi {
        PermissionsApi {
            request: Arc::clone(&self.request),
        }
    }
}
