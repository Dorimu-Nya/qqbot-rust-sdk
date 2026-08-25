use http::Method;
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};

use super::super::super::super::client::{ApiRequestError, ApiRequestExt};
use super::super::super::super::models::err_resp::ErrResp;
use super::DirectApi;

impl DirectApi {
    pub async fn retract_dm_message(
        &self,
        guild_id: &str,
        message_id: &str,
        options: Option<&super::super::models::delete_message_options::DeleteMessageOptions>,
    ) -> Result<(), ApiRequestError<ErrResp>> {
        let guild_id = utf8_percent_encode(guild_id, NON_ALPHANUMERIC);
        let message_id = utf8_percent_encode(message_id, NON_ALPHANUMERIC);
        let path = format!("dms/{guild_id}/messages/{message_id}");
        let mut query = Vec::new();
        if let Some(hidetip) = options.and_then(|options| options.hidetip) {
            query.push(("hidetip", hidetip.to_string()));
        }

        self.request
            .request::<serde_json::Value, (), ErrResp>(&path, Method::DELETE, Some(&query), None)
            .await
    }
}
