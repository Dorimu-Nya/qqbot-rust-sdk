use http::Method;

use super::super::super::super::api_request::{ApiRequestError, ApiRequestExt};
use super::super::super::super::models::err_resp::ErrResp;
use super::RolesApi;

impl RolesApi {
    pub async fn delete_role(
        &self,
        guild_id: &str,
        role_id: &str,
    ) -> Result<serde_json::Value, ApiRequestError<ErrResp>> {
        let path = format!("guilds/{guild_id}/roles/{role_id}");
        self.request
            .request::<serde_json::Value, serde_json::Value, ErrResp>(
                &path,
                Method::DELETE,
                None,
                None,
            )
            .await
    }
}
