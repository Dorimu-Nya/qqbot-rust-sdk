use http::Method;

use super::super::super::super::api_request::{ApiRequestError, ApiRequestExt};
use super::super::super::super::models::err_resp::ErrResp;
use super::models::role_member_action_request::RoleMemberActionRequest;
use super::RolesApi;

impl RolesApi {
    pub async fn member_delete_role(
        &self,
        guild_id: &str,
        role_id: &str,
        user_id: &str,
        body: Option<&RoleMemberActionRequest>,
    ) -> Result<serde_json::Value, ApiRequestError<ErrResp>> {
        let path = format!("guilds/{guild_id}/members/{user_id}/roles/{role_id}");
        self.request
            .request(&path, Method::DELETE, None, body)
            .await
    }
}
