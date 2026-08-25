use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

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

pub async fn put_interaction(
    _interaction_id: &str,
    _body: PutInteractionRequest,
) {
    todo!()
}
