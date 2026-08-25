use http::Method;

use super::super::super::super::api_request::{ApiRequestError, ApiRequestExt};
use super::super::super::super::models::err_resp::ErrResp;
use super::super::super::super::models::guild::Guild;
use super::GuildApi;

impl GuildApi {
    pub async fn guild(&self, guild_id: &str) -> Result<Guild, ApiRequestError<ErrResp>> {
        let path = format!("guilds/{guild_id}");
        self.request
            .request::<serde_json::Value, Guild, ErrResp>(&path, Method::GET, None, None)
            .await
    }
}
