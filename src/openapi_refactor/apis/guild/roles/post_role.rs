use http::Method;

use super::super::super::super::client::{ApiRequestError, ApiRequestExt};
use super::super::super::super::models::err_resp::ErrResp;
use super::models::create_role_request::CreateRoleRequest;
use super::models::create_role_response::CreateRoleResponse;
use super::RolesApi;

impl RolesApi {
    pub async fn post_role(
        &self,
        guild_id: &str,
        body: &CreateRoleRequest,
    ) -> Result<CreateRoleResponse, ApiRequestError<ErrResp>> {
        let path = format!("guilds/{guild_id}/roles");
        self.request
            .request(&path, Method::POST, None, Some(body))
            .await
    }
}
