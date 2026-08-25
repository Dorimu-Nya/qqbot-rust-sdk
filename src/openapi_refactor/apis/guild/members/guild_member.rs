use reqwest::Method;

use super::super::super::super::client::{ApiRequestError, ApiRequestExt};
use super::super::super::super::models::err_resp::ErrResp;
use super::super::super::super::models::member::Member;
use super::MembersApi;

impl MembersApi {
    pub async fn guild_member(
        &self,
        guild_id: &str,
        user_id: &str,
    ) -> Result<Member, ApiRequestError<ErrResp>> {
        let path = format!("guilds/{guild_id}/members/{user_id}");
        self.request
            .request::<serde_json::Value, Member, ErrResp>(&path, Method::GET, None, None)
            .await
    }
}
