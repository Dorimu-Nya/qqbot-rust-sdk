use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OnlineNumsResponse {
    pub online_nums: u32,
}
