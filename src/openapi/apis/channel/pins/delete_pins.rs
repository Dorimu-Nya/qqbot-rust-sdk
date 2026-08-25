use http::Method;

use super::super::super::super::api_request::{ApiRequestError, ApiRequestExt};
use super::super::super::super::models::err_resp::ErrResp;
use super::PinsApi;

impl PinsApi {
    pub async fn delete_pins(
        &self,
        channel_id: &str,
        message_id: &str,
    ) -> Result<serde_json::Value, ApiRequestError<ErrResp>> {
        let path = format!("channels/{channel_id}/pins/{message_id}");
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
