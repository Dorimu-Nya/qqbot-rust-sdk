use http::Method;

use super::super::super::super::api_request::{ApiRequestError, ApiRequestExt};
use super::super::super::super::models::err_resp::ErrResp;
use super::models::menu_put_request::MenuPutRequest;
use super::models::menu_put_response::MenuPutResponse;
use super::MenuApi;

impl MenuApi {
    pub async fn update_menu(
        &self,
        body: &MenuPutRequest,
    ) -> Result<MenuPutResponse, ApiRequestError<ErrResp>> {
        self.request
            .request("v2/menu", Method::PUT, None, Some(body))
            .await
    }
}
