use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use reqwest::Method;

use super::super::super::super::client::{ApiRequestError, ApiRequestExt};
use super::super::super::super::models::err_resp::ErrResp;
use super::models::update_panel_request::UpdatePanelRequest;
use super::models::update_panel_response::UpdatePanelResponse;
use super::PanelApi;

impl PanelApi {
    pub async fn update_panel(
        &self,
        panel_id: &str,
        body: &UpdatePanelRequest,
    ) -> Result<UpdatePanelResponse, ApiRequestError<ErrResp>> {
        let panel_id = utf8_percent_encode(panel_id, NON_ALPHANUMERIC);
        let path = format!("v2/panels/{panel_id}");

        self.request
            .request(&path, Method::PUT, None, Some(body))
            .await
    }
}
