use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MeGuildsQuery {
    #[serde(default)]
    pub before: Option<String>,
    #[serde(default)]
    pub after: Option<String>,
    #[serde(default)]
    pub limit: Option<u32>,
}

pub async fn me_guilds(
    _query: Option<MeGuildsQuery>,
) -> Vec<crate::openapi_refactor::models::Guild> {
    todo!()
}
