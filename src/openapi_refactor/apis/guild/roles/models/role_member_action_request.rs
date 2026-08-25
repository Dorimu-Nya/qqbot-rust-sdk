use serde::{Deserialize, Serialize};

use super::RoleMemberChannel;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoleMemberActionRequest {
    pub channel: RoleMemberChannel,
}
