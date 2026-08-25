use reqwest::Method;

use super::super::super::super::client::{ApiRequestError, ApiRequestExt};
use super::super::super::super::models::err_resp::ErrResp;
use super::models::message::Message;
use super::MessageApi;

impl MessageApi {
    pub async fn message(
        &self,
        channel_id: &str,
        message_id: &str,
    ) -> Result<Message, ApiRequestError<ErrResp>> {
        let path = format!("channels/{channel_id}/messages/{message_id}");
        self.request
            .request::<serde_json::Value, Message, ErrResp>(&path, Method::GET, None, None)
            .await
    }
}
