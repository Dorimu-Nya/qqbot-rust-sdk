use http::Method;

use super::super::super::api_request::{ApiRequestError, ApiRequestExt};
use super::super::super::models::{err_resp::ErrResp, guild::Guild};
use super::UserApis;

impl UserApis {
    pub async fn me_guilds(
        &self,
        before: Option<&str>,
        after: Option<&str>,
        limit: Option<u32>,
    ) -> Result<Vec<Guild>, ApiRequestError<ErrResp>> {
        let mut query = Vec::new();
        if let Some(before) = before {
            query.push(("before", before.to_owned()));
        }
        if let Some(after) = after {
            query.push(("after", after.to_owned()));
        }
        if let Some(limit) = limit {
            query.push(("limit", limit.to_string()));
        }

        self.request
            .request::<serde_json::Value, Vec<Guild>, ErrResp>(
                "users/@me/guilds",
                Method::GET,
                Some(&query),
                None,
            )
            .await
    }
}
