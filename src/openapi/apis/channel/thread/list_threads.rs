use http::Method;

use super::super::super::super::api_request::{ApiRequestError, ApiRequestExt};
use super::super::super::super::models::err_resp::ErrResp;
use super::models::threads_list_response::ThreadsListResponse;
use super::ThreadApi;

impl ThreadApi {
    pub async fn list_threads(
        &self,
        channel_id: &str,
    ) -> Result<ThreadsListResponse, ApiRequestError<ErrResp>> {
        let path = format!("channels/{channel_id}/threads");
        self.request
            .request::<serde_json::Value, ThreadsListResponse, ErrResp>(
                &path,
                Method::GET,
                None,
                None,
            )
            .await
    }
}
