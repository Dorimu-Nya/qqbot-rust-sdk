use serde::{Deserialize, Serialize};

use crate::openapi_refactor::models::Member;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoleMembersResponse {
    pub data: Vec<Member>,
    pub next: String,
}
