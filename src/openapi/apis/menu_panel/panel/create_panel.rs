use http::Method;

use super::super::super::super::api_request::{ApiRequestError, ApiRequestExt};
use super::super::super::super::models::err_resp::ErrResp;
use super::models::create_panel_request::CreatePanelRequest;
use super::models::create_panel_response::CreatePanelResponse;
use super::PanelApi;

impl PanelApi {
    pub async fn create_panel(
        &self,
        body: &CreatePanelRequest,
    ) -> Result<CreatePanelResponse, ApiRequestError<ErrResp>> {
        self.request
            .request("v2/panels", Method::POST, None, Some(body))
            .await
    }
}
