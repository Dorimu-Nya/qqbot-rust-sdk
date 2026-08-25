use http::Method;

use super::super::super::super::client::{ApiRequestError, ApiRequestExt};
use super::super::super::super::models::err_resp::ErrResp;
use super::models::api_permissions_response::ApiPermissionsResponse;
use super::PermissionsApi;

impl PermissionsApi {
    pub async fn get_api_permissions(
        &self,
        guild_id: &str,
    ) -> Result<ApiPermissionsResponse, ApiRequestError<ErrResp>> {
        let path = format!("guilds/{guild_id}/api_permission");
        self.request
            .request::<serde_json::Value, ApiPermissionsResponse, ErrResp>(
                &path,
                Method::GET,
                None,
                None,
            )
            .await
    }
}
