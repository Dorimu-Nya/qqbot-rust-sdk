use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use reqwest::Method;

use super::super::super::super::client::{ApiRequestError, ApiRequestExt};
use super::super::super::super::models::err_resp::ErrResp;
use super::DirectApi;

impl DirectApi {
    pub async fn post_direct_message(
        &self,
        guild_id: &str,
        body: &super::super::models::send_message_request::SendMessageRequest,
    ) -> Result<
        super::super::models::send_message_response::SendMessageResponse,
        ApiRequestError<ErrResp>,
    > {
        let guild_id = utf8_percent_encode(guild_id, NON_ALPHANUMERIC);
        let path = format!("dms/{guild_id}/messages");

        self.request
            .request(&path, Method::POST, None, Some(body))
            .await
    }
}
