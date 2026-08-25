use std::sync::Arc;

use super::super::super::api_request::ApiRequest;
use super::MenuPanelApis;

mod get_menu;
pub mod models;
mod update_menu;

pub struct MenuApi {
    pub(crate) request: Arc<dyn ApiRequest>,
}

impl MenuPanelApis {
    pub fn menu(&self) -> MenuApi {
        MenuApi {
            request: Arc::clone(&self.request),
        }
    }
}
