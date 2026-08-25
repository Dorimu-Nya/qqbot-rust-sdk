use http::Method;

use super::super::super::super::api_request::{ApiRequestError, ApiRequestExt};
use super::super::super::super::models::err_resp::ErrResp;
use super::models::menu_response::MenuResponse;
use super::MenuApi;

impl MenuApi {
    pub async fn get_menu(&self) -> Result<MenuResponse, ApiRequestError<ErrResp>> {
        self.request
            .request::<serde_json::Value, MenuResponse, ErrResp>("v2/menu", Method::GET, None, None)
            .await
    }
}
