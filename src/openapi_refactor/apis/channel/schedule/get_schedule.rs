use http::Method;

use super::super::super::super::client::{ApiRequestError, ApiRequestExt};
use super::super::super::super::models::err_resp::ErrResp;
use super::models::schedule::Schedule;
use super::ScheduleApi;

impl ScheduleApi {
    pub async fn get_schedule(
        &self,
        channel_id: &str,
        schedule_id: &str,
    ) -> Result<Schedule, ApiRequestError<ErrResp>> {
        let path = format!("channels/{channel_id}/schedules/{schedule_id}");
        self.request
            .request::<serde_json::Value, Schedule, ErrResp>(&path, Method::GET, None, None)
            .await
    }
}
