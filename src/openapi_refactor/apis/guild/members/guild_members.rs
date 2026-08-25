use http::Method;

use super::super::super::super::client::{ApiRequestError, ApiRequestExt};
use super::super::super::super::models::err_resp::ErrResp;
use super::super::super::super::models::member::Member;
use super::MembersApi;

impl MembersApi {
    pub async fn guild_members(
        &self,
        guild_id: &str,
        after: Option<&str>,
        limit: Option<u32>,
    ) -> Result<Vec<Member>, ApiRequestError<ErrResp>> {
        let path = format!("guilds/{guild_id}/members");
        let mut query = Vec::new();
        if let Some(after) = after {
            query.push(("after", after.to_owned()));
        }
        if let Some(limit) = limit {
            query.push(("limit", limit.to_string()));
        }

        self.request
            .request::<serde_json::Value, Vec<Member>, ErrResp>(
                &path,
                Method::GET,
                Some(&query),
                None,
            )
            .await
    }
}
