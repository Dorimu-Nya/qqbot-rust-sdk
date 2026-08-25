use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiPermission {
    pub path: String,
    pub method: String,
    pub desc: String,
    pub auth_status: i32,
}
