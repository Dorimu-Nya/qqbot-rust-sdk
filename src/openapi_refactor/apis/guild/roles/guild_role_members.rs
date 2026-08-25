use reqwest::Method;

use super::super::super::super::client::{ApiRequestError, ApiRequestExt};
use super::super::super::super::models::err_resp::ErrResp;
use super::models::role_members_response::RoleMembersResponse;
use super::RolesApi;

impl RolesApi {
    pub async fn guild_role_members(
        &self,
        guild_id: &str,
        role_id: &str,
        start_index: Option<&str>,
        limit: Option<u32>,
    ) -> Result<RoleMembersResponse, ApiRequestError<ErrResp>> {
        let path = format!("guilds/{guild_id}/roles/{role_id}/members");
        let mut query = Vec::new();
        if let Some(start_index) = start_index {
            query.push(("start_index", start_index.to_owned()));
        }
        if let Some(limit) = limit {
            query.push(("limit", limit.to_string()));
        }

        self.request
            .request::<serde_json::Value, RoleMembersResponse, ErrResp>(
                &path,
                Method::GET,
                Some(&query),
                None,
            )
            .await
    }
}
