use http::Method;

use super::super::super::super::client::{ApiRequestError, ApiRequestExt};
use super::super::super::super::models::channel::Channel;
use super::super::super::super::models::err_resp::ErrResp;
use super::ChannelApi;

impl ChannelApi {
    pub async fn channel(&self, channel_id: &str) -> Result<Channel, ApiRequestError<ErrResp>> {
        let path = format!("channels/{channel_id}");
        self.request
            .request::<serde_json::Value, Channel, ErrResp>(&path, Method::GET, None, None)
            .await
    }
}
