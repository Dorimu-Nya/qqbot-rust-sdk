use std::sync::Arc;

use super::super::api::QQApiClient;
use super::super::api_request::ApiRequest;

pub mod menu;
pub mod panel;

pub struct MenuPanelApis {
    pub(crate) request: Arc<dyn ApiRequest>,
}

impl QQApiClient {
    pub fn menu_panel(&self) -> MenuPanelApis {
        MenuPanelApis {
            request: Arc::clone(&self.request),
        }
    }
}
