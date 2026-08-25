use http::Method;
use serde::{Deserialize, Serialize};

use super::super::super::super::api_request::{ApiRequestError, ApiRequestExt};
use super::super::super::super::models::err_resp::ErrResp;
use super::AudioApi;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PostAudioRequest {
    #[serde(default)]
    pub audio_url: Option<String>,
    #[serde(default)]
    pub text: Option<String>,
    #[serde(default)]
    pub status: Option<i32>,
}

impl AudioApi {
    pub async fn post_audio(
        &self,
        channel_id: &str,
        body: &PostAudioRequest,
    ) -> Result<serde_json::Value, ApiRequestError<ErrResp>> {
        let path = format!("channels/{channel_id}/audio");
        self.request
            .request(&path, Method::POST, None, Some(body))
            .await
    }
}
