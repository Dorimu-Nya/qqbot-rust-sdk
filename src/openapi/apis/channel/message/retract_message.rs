use http::Method;

use super::super::super::super::api_request::{ApiRequestError, ApiRequestExt};
use super::super::super::super::models::err_resp::ErrResp;
use super::super::super::message::models::delete_message_options::DeleteMessageOptions;
use super::MessageApi;

impl MessageApi {
    pub async fn retract_message(
        &self,
        channel_id: &str,
        message_id: &str,
        options: Option<&DeleteMessageOptions>,
    ) -> Result<serde_json::Value, ApiRequestError<ErrResp>> {
        let path = format!("channels/{channel_id}/messages/{message_id}");
        let mut query = Vec::new();
        if let Some(hidetip) = options.and_then(|options| options.hidetip) {
            query.push(("hidetip", hidetip.to_string()));
        }
        self.request
            .request::<serde_json::Value, serde_json::Value, ErrResp>(
                &path,
                Method::DELETE,
                Some(&query),
                None,
            )
            .await
    }
}
