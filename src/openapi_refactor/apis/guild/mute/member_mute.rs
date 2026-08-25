use reqwest::Method;

use super::super::super::super::client::{ApiRequestError, ApiRequestExt};
use super::super::super::super::models::err_resp::ErrResp;
use super::models::guild_mute_request::GuildMuteRequest;
use super::MuteApi;

impl MuteApi {
    pub async fn member_mute(
        &self,
        guild_id: &str,
        user_id: &str,
        body: &GuildMuteRequest,
    ) -> Result<serde_json::Value, ApiRequestError<ErrResp>> {
        let path = format!("guilds/{guild_id}/members/{user_id}/mute");
        self.request
            .request(&path, Method::PATCH, None, Some(body))
            .await
    }
}
