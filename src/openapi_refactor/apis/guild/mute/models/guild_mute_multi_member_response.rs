use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GuildMuteMultiMemberResponse {
    #[serde(default)]
    pub user_ids: Vec<String>,
}
