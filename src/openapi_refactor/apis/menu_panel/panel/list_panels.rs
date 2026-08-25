use http::Method;

use super::super::super::super::client::{ApiRequestError, ApiRequestExt};
use super::super::super::super::models::err_resp::ErrResp;
use super::models::panels_response::PanelsResponse;
use super::PanelApi;

impl PanelApi {
    pub async fn list_panels(
        &self,
        scope: &str,
        cursor: Option<&str>,
        limit: Option<u32>,
    ) -> Result<PanelsResponse, ApiRequestError<ErrResp>> {
        let mut query = vec![("scope", scope.to_owned())];
        if let Some(cursor) = cursor {
            query.push(("cursor", cursor.to_owned()));
        }
        if let Some(limit) = limit {
            query.push(("limit", limit.to_string()));
        }

        self.request
            .request::<serde_json::Value, PanelsResponse, ErrResp>(
                "v2/panels",
                Method::GET,
                Some(&query),
                None,
            )
            .await
    }
}
