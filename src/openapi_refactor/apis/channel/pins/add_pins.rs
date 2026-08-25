use reqwest::Method;

use super::super::super::super::client::{ApiRequestError, ApiRequestExt};
use super::super::super::super::models::err_resp::ErrResp;
use super::models::pins_message::PinsMessage;
use super::PinsApi;

impl PinsApi {
    pub async fn add_pins(
        &self,
        channel_id: &str,
        message_id: &str,
    ) -> Result<PinsMessage, ApiRequestError<ErrResp>> {
        let path = format!("channels/{channel_id}/pins/{message_id}");
        self.request
            .request::<serde_json::Value, PinsMessage, ErrResp>(&path, Method::PUT, None, None)
            .await
    }
}
