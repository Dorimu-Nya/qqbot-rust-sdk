use reqwest::Method;

use super::super::super::super::client::{ApiRequestError, ApiRequestExt};
use super::super::super::super::models::err_resp::ErrResp;
use super::AnnouncesApi;

impl AnnouncesApi {
    pub async fn clean_channel_announces(
        &self,
        channel_id: &str,
    ) -> Result<serde_json::Value, ApiRequestError<ErrResp>> {
        let path = format!("channels/{channel_id}/announces/all");
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
