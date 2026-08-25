use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ReactionUsersQuery {
    #[serde(default)]
    pub cookie: Option<String>,
    #[serde(default)]
    pub limit: Option<u32>,
}
