use http::Method;

use super::super::super::super::client::{ApiRequestError, ApiRequestExt};
use super::super::super::super::models::err_resp::ErrResp;
use super::models::api_permission_demand::ApiPermissionDemand;
use super::models::create_api_permission_demand_request::CreateApiPermissionDemandRequest;
use super::PermissionsApi;

impl PermissionsApi {
    pub async fn require_api_permissions(
        &self,
        guild_id: &str,
        body: &CreateApiPermissionDemandRequest,
    ) -> Result<ApiPermissionDemand, ApiRequestError<ErrResp>> {
        let path = format!("guilds/{guild_id}/api_permission/demand");
        self.request
            .request(&path, Method::POST, None, Some(body))
            .await
    }
}
