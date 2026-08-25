use http::Method;

use super::super::super::super::client::{ApiRequestError, ApiRequestExt};
use super::super::super::super::models::err_resp::ErrResp;
use super::models::online_nums_response::OnlineNumsResponse;
use super::ChannelApi;

impl ChannelApi {
    pub async fn online_nums(
        &self,
        channel_id: &str,
    ) -> Result<OnlineNumsResponse, ApiRequestError<ErrResp>> {
        let path = format!("channels/{channel_id}/online_nums");
        self.request
            .request::<serde_json::Value, OnlineNumsResponse, ErrResp>(
                &path,
                Method::GET,
                None,
                None,
            )
            .await
    }
}
