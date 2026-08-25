use http::Method;

use super::super::super::super::client::{ApiRequestError, ApiRequestExt};
use super::super::super::super::models::err_resp::ErrResp;
use super::models::create_thread_request::CreateThreadRequest;
use super::models::create_thread_response::CreateThreadResponse;
use super::ThreadApi;

impl ThreadApi {
    pub async fn create_thread(
        &self,
        channel_id: &str,
        body: &CreateThreadRequest,
    ) -> Result<CreateThreadResponse, ApiRequestError<ErrResp>> {
        let path = format!("channels/{channel_id}/threads");
        self.request
            .request(&path, Method::PUT, None, Some(body))
            .await
    }
}
