use std::sync::Arc;

use super::super::super::api_request::ApiRequest;
use super::ChannelApis;

mod channel_permissions;
mod channel_roles_permissions;
pub mod models;
mod put_channel_permissions;
mod put_channel_roles_permissions;

pub struct PermissionsApi {
    pub(crate) request: Arc<dyn ApiRequest>,
}

impl ChannelApis {
    pub fn permissions(&self) -> PermissionsApi {
        PermissionsApi {
            request: Arc::clone(&self.request),
        }
    }
}
