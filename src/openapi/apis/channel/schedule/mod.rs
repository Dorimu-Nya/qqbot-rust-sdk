use std::sync::Arc;

use super::super::super::api_request::ApiRequest;
use super::ChannelApis;

mod create_schedule;
mod delete_schedule;
mod get_schedule;
mod list_schedules;
pub mod models;
mod modify_schedule;

pub struct ScheduleApi {
    pub(crate) request: Arc<dyn ApiRequest>,
}

impl ChannelApis {
    pub fn schedule(&self) -> ScheduleApi {
        ScheduleApi {
            request: Arc::clone(&self.request),
        }
    }
}
