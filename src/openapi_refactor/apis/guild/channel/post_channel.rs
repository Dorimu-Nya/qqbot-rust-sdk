use reqwest::Method;

use super::super::super::super::client::{ApiRequestError, ApiRequestExt};
use super::super::super::super::models::channel::Channel;
use super::super::super::super::models::err_resp::ErrResp;
use super::models::create_channel_request::CreateChannelRequest;
use super::ChannelApi;

impl ChannelApi {
    pub async fn post_channel(
        &self,
        guild_id: &str,
        body: &CreateChannelRequest,
    ) -> Result<Channel, ApiRequestError<ErrResp>> {
        let path = format!("guilds/{guild_id}/channels");
        self.request
            .request(&path, Method::POST, None, Some(body))
            .await
    }
}
