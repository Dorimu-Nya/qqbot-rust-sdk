use http::Method;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use super::super::super::api_request::{ApiRequestError, ApiRequestExt};
use super::super::super::models::err_resp::ErrResp;
use super::UserApis;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDirectMessageRequest {
    pub recipient_id: String,
    pub source_guild_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateDirectMessageResponse {
    #[serde(default)]
    pub guild_id: Option<String>,
    #[serde(default)]
    pub channel_id: Option<String>,
    #[serde(default)]
    pub create_time: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

impl UserApis {
    pub async fn create_direct_message(
        &self,
        body: &CreateDirectMessageRequest,
    ) -> Result<CreateDirectMessageResponse, ApiRequestError<ErrResp>> {
        self.request
            .request("users/@me/dms", Method::POST, None, Some(body))
            .await
    }
}
