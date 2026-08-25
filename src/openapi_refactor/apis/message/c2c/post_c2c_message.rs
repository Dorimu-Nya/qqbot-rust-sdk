use http::Method;
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};

use super::super::super::super::client::{ApiRequestError, ApiRequestExt};
use super::super::super::super::models::err_resp::ErrResp;
use super::C2cApi;

impl C2cApi {
    pub async fn post_c2c_message(
        &self,
        openid: &str,
        body: &super::super::models::send_message_request::SendMessageRequest,
    ) -> Result<
        super::super::models::send_message_response::SendMessageResponse,
        ApiRequestError<ErrResp>,
    > {
        let openid = utf8_percent_encode(openid, NON_ALPHANUMERIC);
        let path = format!("v2/users/{openid}/messages");

        self.request
            .request(&path, Method::POST, None, Some(body))
            .await
    }
}
