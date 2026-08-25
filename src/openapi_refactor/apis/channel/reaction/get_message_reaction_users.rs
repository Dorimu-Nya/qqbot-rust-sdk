use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use reqwest::Method;

use super::super::super::super::client::{ApiRequestError, ApiRequestExt};
use super::super::super::super::models::err_resp::ErrResp;
use super::models::reaction_users_response::ReactionUsersResponse;
use super::ReactionApi;

impl ReactionApi {
    pub async fn get_message_reaction_users(
        &self,
        channel_id: &str,
        message_id: &str,
        reaction_type: u32,
        reaction_id: &str,
        cookie: Option<&str>,
        limit: Option<u32>,
    ) -> Result<ReactionUsersResponse, ApiRequestError<ErrResp>> {
        let reaction_id = utf8_percent_encode(reaction_id, NON_ALPHANUMERIC);
        let path = format!(
            "channels/{channel_id}/messages/{message_id}/reactions/{reaction_type}/{reaction_id}"
        );
        let mut query = Vec::new();
        if let Some(cookie) = cookie {
            query.push(("cookie", cookie.to_owned()));
        }
        if let Some(limit) = limit {
            query.push(("limit", limit.to_string()));
        }

        self.request
            .request::<serde_json::Value, ReactionUsersResponse, ErrResp>(
                &path,
                Method::GET,
                Some(&query),
                None,
            )
            .await
    }
}
