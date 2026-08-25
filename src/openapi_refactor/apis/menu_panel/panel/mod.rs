use std::sync::Arc;

use super::super::super::api_request::ApiRequest;
use super::MenuPanelApis;

mod create_panel;
mod delete_panel;
mod get_panel;
mod list_panels;
pub mod models;
mod update_panel;
mod update_panel_target;

pub struct PanelApi {
    pub(crate) request: Arc<dyn ApiRequest>,
}

impl MenuPanelApis {
    pub fn panel(&self) -> PanelApi {
        PanelApi {
            request: Arc::clone(&self.request),
        }
    }
}
