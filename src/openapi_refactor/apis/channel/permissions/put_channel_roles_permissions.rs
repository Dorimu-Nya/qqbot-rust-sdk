use reqwest::Method;

use super::super::super::super::client::{ApiRequestError, ApiRequestExt};
use super::super::super::super::models::err_resp::ErrResp;
use super::models::modify_channel_permissions_request::ModifyChannelPermissionsRequest;
use super::PermissionsApi;

impl PermissionsApi {
    pub async fn put_channel_roles_permissions(
        &self,
        channel_id: &str,
        role_id: &str,
        body: &ModifyChannelPermissionsRequest,
    ) -> Result<serde_json::Value, ApiRequestError<ErrResp>> {
        let path = format!("channels/{channel_id}/roles/{role_id}/permissions");
        self.request
            .request(&path, Method::PUT, None, Some(body))
            .await
    }
}
