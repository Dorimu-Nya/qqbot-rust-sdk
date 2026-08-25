use http::Method;
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};

use super::super::super::super::client::{ApiRequestError, ApiRequestExt};
use super::super::super::super::models::err_resp::ErrResp;
use super::PanelApi;

impl PanelApi {
    pub async fn delete_panel(
        &self,
        panel_id: &str,
    ) -> Result<serde_json::Value, ApiRequestError<ErrResp>> {
        let panel_id = utf8_percent_encode(panel_id, NON_ALPHANUMERIC);
        let path = format!("v2/panels/{panel_id}");

        self.request
            .request::<serde_json::Value, serde_json::Value, ErrResp>(
                &path,
                Method::DELETE,
                None,
                None,
            )
            .await
    }
}
