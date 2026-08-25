use serde::{Deserialize, Serialize};

use super::super::super::super::super::models::member::Member;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoleMembersResponse {
    pub data: Vec<Member>,
    pub next: String,
}
