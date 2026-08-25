use http::Method;
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};

use super::super::super::super::api_request::{ApiRequestError, ApiRequestExt};
use super::super::super::super::models::err_resp::ErrResp;
use super::ReactionApi;

impl ReactionApi {
    pub async fn create_message_reaction(
        &self,
        channel_id: &str,
        message_id: &str,
        reaction_type: u32,
        reaction_id: &str,
    ) -> Result<serde_json::Value, ApiRequestError<ErrResp>> {
        let reaction_id = utf8_percent_encode(reaction_id, NON_ALPHANUMERIC);
        let path = format!(
            "channels/{channel_id}/messages/{message_id}/reactions/{reaction_type}/{reaction_id}"
        );
        self.request
            .request::<serde_json::Value, serde_json::Value, ErrResp>(
                &path,
                Method::PUT,
                None,
                None,
            )
            .await
    }
}
