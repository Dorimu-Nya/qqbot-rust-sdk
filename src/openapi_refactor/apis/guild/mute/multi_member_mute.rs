use http::Method;

use super::super::super::super::client::{ApiRequestError, ApiRequestExt};
use super::super::super::super::models::err_resp::ErrResp;
use super::models::guild_mute_multi_member_request::GuildMuteMultiMemberRequest;
use super::models::guild_mute_multi_member_response::GuildMuteMultiMemberResponse;
use super::MuteApi;

impl MuteApi {
    pub async fn multi_member_mute(
        &self,
        guild_id: &str,
        body: &GuildMuteMultiMemberRequest,
    ) -> Result<GuildMuteMultiMemberResponse, ApiRequestError<ErrResp>> {
        let path = format!("guilds/{guild_id}/mute");
        self.request
            .request(&path, Method::PATCH, None, Some(body))
            .await
    }
}
