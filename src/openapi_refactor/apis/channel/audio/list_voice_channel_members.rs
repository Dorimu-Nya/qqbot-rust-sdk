use reqwest::Method;

use super::super::super::super::client::{ApiRequestError, ApiRequestExt};
use super::super::super::super::models::err_resp::ErrResp;
use super::super::super::super::models::member::Member;
use super::AudioApi;

impl AudioApi {
    pub async fn list_voice_channel_members(
        &self,
        channel_id: &str,
    ) -> Result<Vec<Member>, ApiRequestError<ErrResp>> {
        let path = format!("channels/{channel_id}/voice/members");
        self.request
            .request::<serde_json::Value, Vec<Member>, ErrResp>(&path, Method::GET, None, None)
            .await
    }
}
