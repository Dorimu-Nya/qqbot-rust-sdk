use http::Method;
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};

use super::super::super::super::client::{ApiRequestError, ApiRequestExt};
use super::super::super::super::models::err_resp::ErrResp;
use super::GroupApi;

impl GroupApi {
    pub async fn post_group_message(
        &self,
        group_openid: &str,
        body: &super::super::models::send_message_request::SendMessageRequest,
    ) -> Result<
        super::super::models::send_message_response::SendMessageResponse,
        ApiRequestError<ErrResp>,
    > {
        let group_openid = utf8_percent_encode(group_openid, NON_ALPHANUMERIC);
        let path = format!("v2/groups/{group_openid}/messages");

        self.request
            .request(&path, Method::POST, None, Some(body))
            .await
    }
}
