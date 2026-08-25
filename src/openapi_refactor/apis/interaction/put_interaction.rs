use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use super::super::super::client::{ApiRequestError, ApiRequestExt};
use super::super::super::models::err_resp::ErrResp;
use super::InteractionApis;

#[derive(Debug, Clone, Copy, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum InteractionAckCode {
    Success = 0,
    Failed = 1,
    TooFrequent = 2,
    Duplicated = 3,
    NoPermission = 4,
    AdminOnly = 5,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PutInteractionRequest {
    pub code: InteractionAckCode,
}

impl InteractionApis {
    pub async fn put_interaction(
        &self,
        interaction_id: &str,
        body: &PutInteractionRequest,
    ) -> Result<serde_json::Value, ApiRequestError<ErrResp>> {
        let path = format!("interactions/{interaction_id}");

        self.request
            .request(&path, Method::PUT, None, Some(body))
            .await
    }
}
