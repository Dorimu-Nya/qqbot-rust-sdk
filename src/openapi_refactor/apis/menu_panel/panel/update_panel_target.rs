use http::Method;
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};

use super::super::super::super::client::{ApiRequestError, ApiRequestExt};
use super::super::super::super::models::err_resp::ErrResp;
use super::models::update_panel_target_request::UpdatePanelTargetRequest;
use super::PanelApi;

impl PanelApi {
    pub async fn update_panel_target(
        &self,
        panel_id: &str,
        body: &UpdatePanelTargetRequest,
    ) -> Result<serde_json::Value, ApiRequestError<ErrResp>> {
        let panel_id = utf8_percent_encode(panel_id, NON_ALPHANUMERIC);
        let path = format!("v2/panels/{panel_id}/target");

        self.request
            .request(&path, Method::PUT, None, Some(body))
            .await
    }
}
