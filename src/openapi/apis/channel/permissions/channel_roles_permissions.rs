use http::Method;

use super::super::super::super::api_request::{ApiRequestError, ApiRequestExt};
use super::super::super::super::models::err_resp::ErrResp;
use super::models::channel_permissions::ChannelPermissions;
use super::PermissionsApi;

impl PermissionsApi {
    pub async fn channel_roles_permissions(
        &self,
        channel_id: &str,
        role_id: &str,
    ) -> Result<ChannelPermissions, ApiRequestError<ErrResp>> {
        let path = format!("channels/{channel_id}/roles/{role_id}/permissions");
        self.request
            .request::<serde_json::Value, ChannelPermissions, ErrResp>(
                &path,
                Method::GET,
                None,
                None,
            )
            .await
    }
}
