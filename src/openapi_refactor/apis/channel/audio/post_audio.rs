use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PostAudioRequest {
    #[serde(default)]
    pub audio_url: Option<String>,
    #[serde(default)]
    pub text: Option<String>,
    #[serde(default)]
    pub status: Option<i32>,
}

pub async fn post_audio(_channel_id: &str, _body: PostAudioRequest) {
    todo!()
}
