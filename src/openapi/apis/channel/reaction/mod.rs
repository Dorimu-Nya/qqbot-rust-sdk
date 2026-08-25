use std::sync::Arc;

use super::super::super::api_request::ApiRequest;
use super::ChannelApis;

mod create_message_reaction;
mod delete_own_message_reaction;
mod get_message_reaction_users;
pub mod models;

pub struct ReactionApi {
    pub(crate) request: Arc<dyn ApiRequest>,
}

impl ChannelApis {
    pub fn reaction(&self) -> ReactionApi {
        ReactionApi {
            request: Arc::clone(&self.request),
        }
    }
}
