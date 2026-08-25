use http::Method;

use super::super::super::super::client::{ApiRequestError, ApiRequestExt};
use super::super::super::super::models::channel::Channel;
use super::super::super::super::models::err_resp::ErrResp;
use super::ChannelApi;

impl ChannelApi {
    pub async fn channels(&self, guild_id: &str) -> Result<Vec<Channel>, ApiRequestError<ErrResp>> {
        let path = format!("guilds/{guild_id}/channels");
        self.request
            .request::<serde_json::Value, Vec<Channel>, ErrResp>(&path, Method::GET, None, None)
            .await
    }
}
