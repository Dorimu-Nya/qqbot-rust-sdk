use http::Method;

use super::super::super::super::client::{ApiRequestError, ApiRequestExt};
use super::super::super::super::models::err_resp::ErrResp;
use super::models::update_role_request::UpdateRoleRequest;
use super::models::update_role_response::UpdateRoleResponse;
use super::RolesApi;

impl RolesApi {
    pub async fn patch_role(
        &self,
        guild_id: &str,
        role_id: &str,
        body: &UpdateRoleRequest,
    ) -> Result<UpdateRoleResponse, ApiRequestError<ErrResp>> {
        let path = format!("guilds/{guild_id}/roles/{role_id}");
        self.request
            .request(&path, Method::PATCH, None, Some(body))
            .await
    }
}
