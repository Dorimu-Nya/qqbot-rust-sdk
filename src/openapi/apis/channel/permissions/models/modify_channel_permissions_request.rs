use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModifyChannelPermissionsRequest {
    pub add: String,
    pub remove: String,
}
