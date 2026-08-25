use reqwest::Method;

use super::super::super::super::client::{ApiRequestError, ApiRequestExt};
use super::super::super::super::models::err_resp::ErrResp;
use super::models::thread_detail_response::ThreadDetailResponse;
use super::ThreadApi;

impl ThreadApi {
    pub async fn get_thread(
        &self,
        channel_id: &str,
        thread_id: &str,
    ) -> Result<ThreadDetailResponse, ApiRequestError<ErrResp>> {
        let path = format!("channels/{channel_id}/threads/{thread_id}");
        self.request
            .request::<serde_json::Value, ThreadDetailResponse, ErrResp>(
                &path,
                Method::GET,
                None,
                None,
            )
            .await
    }
}
