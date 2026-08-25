use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use reqwest::Method;

use super::super::super::super::client::{ApiRequestError, ApiRequestExt};
use super::super::super::super::models::err_resp::ErrResp;
use super::models::panel_detail_response::PanelDetailResponse;
use super::PanelApi;

impl PanelApi {
    pub async fn get_panel(
        &self,
        panel_id: &str,
    ) -> Result<PanelDetailResponse, ApiRequestError<ErrResp>> {
        let panel_id = utf8_percent_encode(panel_id, NON_ALPHANUMERIC);
        let path = format!("v2/panels/{panel_id}");

        self.request
            .request::<serde_json::Value, PanelDetailResponse, ErrResp>(
                &path,
                Method::GET,
                None,
                None,
            )
            .await
    }
}
