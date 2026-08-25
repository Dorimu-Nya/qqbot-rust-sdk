use serde::{Deserialize, Serialize};

use super::user::User;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Member {
    pub user: User,
    #[serde(default)]
    pub nick: Option<String>,
    #[serde(default)]
    pub roles: Vec<String>,
    #[serde(default)]
    pub joined_at: Option<String>,
    #[serde(default)]
    pub deaf: Option<bool>,
    #[serde(default)]
    pub mute: Option<bool>,
    #[serde(default)]
    pub pending: Option<bool>,
}
