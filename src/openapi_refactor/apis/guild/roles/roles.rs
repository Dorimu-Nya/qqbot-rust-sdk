use http::Method;

use super::super::super::super::client::{ApiRequestError, ApiRequestExt};
use super::super::super::super::models::err_resp::ErrResp;
use super::models::guild_roles_response::GuildRolesResponse;
use super::RolesApi;

impl RolesApi {
    pub async fn roles(
        &self,
        guild_id: &str,
    ) -> Result<GuildRolesResponse, ApiRequestError<ErrResp>> {
        let path = format!("guilds/{guild_id}/roles");
        self.request
            .request::<serde_json::Value, GuildRolesResponse, ErrResp>(
                &path,
                Method::GET,
                None,
                None,
            )
            .await
    }
}
