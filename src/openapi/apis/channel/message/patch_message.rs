use http::Method;

use super::super::super::super::api_request::{ApiRequestError, ApiRequestExt};
use super::super::super::super::models::err_resp::ErrResp;
use super::super::super::message::models::send_message_response::SendMessageResponse;
use super::models::update_message_request::UpdateMessageRequest;
use super::MessageApi;

impl MessageApi {
    pub async fn patch_message(
        &self,
        channel_id: &str,
        message_id: &str,
        body: &UpdateMessageRequest,
    ) -> Result<SendMessageResponse, ApiRequestError<ErrResp>> {
        let path = format!("channels/{channel_id}/messages/{message_id}");
        self.request
            .request(&path, Method::PATCH, None, Some(body))
            .await
    }
}
