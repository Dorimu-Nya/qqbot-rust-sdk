use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use reqwest::Method;

use super::super::super::super::client::{ApiRequestError, ApiRequestExt};
use super::super::super::super::models::err_resp::ErrResp;
use super::GroupApi;

impl GroupApi {
    pub async fn retract_group_message(
        &self,
        group_openid: &str,
        message_id: &str,
    ) -> Result<(), ApiRequestError<ErrResp>> {
        let group_openid = utf8_percent_encode(group_openid, NON_ALPHANUMERIC);
        let message_id = utf8_percent_encode(message_id, NON_ALPHANUMERIC);
        let path = format!("v2/groups/{group_openid}/messages/{message_id}");

        self.request
            .request::<serde_json::Value, (), ErrResp>(&path, Method::DELETE, None, None)
            .await
    }
}
