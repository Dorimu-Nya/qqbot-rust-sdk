use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use super::super::super::super::super::models::user::User;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ReactionUsersResponse {
    #[serde(default)]
    pub users: Vec<User>,
    #[serde(default)]
    pub is_end: Option<bool>,
    #[serde(default)]
    pub cookie: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}
