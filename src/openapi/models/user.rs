use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub username: String,
    #[serde(default)]
    pub avatar: Option<String>,
    #[serde(default)]
    pub bot: Option<bool>,
    #[serde(default)]
    pub union_openid: Option<String>,
    #[serde(default)]
    pub union_user_account: Option<String>,
    #[serde(default)]
    pub public_flags: Option<u64>,
    #[serde(default)]
    pub system: Option<bool>,
}
