use http::Method;
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};

use super::super::super::super::api_request::{ApiRequestError, ApiRequestExt};
use super::super::super::super::models::err_resp::ErrResp;
use super::C2cApi;

impl C2cApi {
    pub async fn retract_c2c_message(
        &self,
        openid: &str,
        message_id: &str,
    ) -> Result<(), ApiRequestError<ErrResp>> {
        let openid = utf8_percent_encode(openid, NON_ALPHANUMERIC);
        let message_id = utf8_percent_encode(message_id, NON_ALPHANUMERIC);
        let path = format!("v2/users/{openid}/messages/{message_id}");

        self.request
            .request::<serde_json::Value, (), ErrResp>(&path, Method::DELETE, None, None)
            .await
    }
}
