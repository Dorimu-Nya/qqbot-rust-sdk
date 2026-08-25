use serde::{Deserialize, Serialize};

use super::role_member_channel::RoleMemberChannel;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoleMemberActionRequest {
    pub channel: RoleMemberChannel,
}
