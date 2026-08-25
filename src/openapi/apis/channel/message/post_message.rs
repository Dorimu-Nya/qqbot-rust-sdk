use http::Method;

use super::super::super::super::api_request::{ApiRequestError, ApiRequestExt};
use super::super::super::super::models::err_resp::ErrResp;
use super::super::super::message::models::send_message_request::SendMessageRequest;
use super::super::super::message::models::send_message_response::SendMessageResponse;
use super::MessageApi;

impl MessageApi {
    pub async fn post_message(
        &self,
        channel_id: &str,
        body: &SendMessageRequest,
    ) -> Result<SendMessageResponse, ApiRequestError<ErrResp>> {
        let path = format!("channels/{channel_id}/messages");
        self.request
            .request(&path, Method::POST, None, Some(body))
            .await
    }
}
