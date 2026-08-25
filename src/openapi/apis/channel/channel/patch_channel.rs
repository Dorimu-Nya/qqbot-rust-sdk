use http::Method;

use super::super::super::super::api_request::{ApiRequestError, ApiRequestExt};
use super::super::super::super::models::channel::Channel;
use super::super::super::super::models::err_resp::ErrResp;
use super::models::update_channel_request::UpdateChannelRequest;
use super::ChannelApi;

impl ChannelApi {
    pub async fn patch_channel(
        &self,
        channel_id: &str,
        body: &UpdateChannelRequest,
    ) -> Result<Channel, ApiRequestError<ErrResp>> {
        let path = format!("channels/{channel_id}");
        self.request
            .request(&path, Method::PATCH, None, Some(body))
            .await
    }
}
