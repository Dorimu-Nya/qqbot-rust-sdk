use http::Method;

use super::super::super::super::client::{ApiRequestError, ApiRequestExt};
use super::super::super::super::models::err_resp::ErrResp;
use super::models::delete_member_options::DeleteMemberOptions;
use super::MembersApi;

impl MembersApi {
    pub async fn delete_guild_member(
        &self,
        guild_id: &str,
        user_id: &str,
        options: Option<&DeleteMemberOptions>,
    ) -> Result<serde_json::Value, ApiRequestError<ErrResp>> {
        let path = format!("guilds/{guild_id}/members/{user_id}");
        let mut query = Vec::new();
        if let Some(options) = options {
            if let Some(add_blacklist) = options.add_blacklist {
                query.push(("add_blacklist", add_blacklist.to_string()));
            }
            if let Some(days) = options.delete_history_msg_days {
                query.push(("delete_history_msg_days", days.to_string()));
            }
        }

        self.request
            .request::<serde_json::Value, serde_json::Value, ErrResp>(
                &path,
                Method::DELETE,
                Some(&query),
                None,
            )
            .await
    }
}
