use http::Method;

use super::super::super::super::client::{ApiRequestError, ApiRequestExt};
use super::super::super::super::models::err_resp::ErrResp;
use super::models::schedule::Schedule;
use super::models::upsert_schedule_request::UpsertScheduleRequest;
use super::ScheduleApi;

impl ScheduleApi {
    pub async fn modify_schedule(
        &self,
        channel_id: &str,
        schedule_id: &str,
        body: &UpsertScheduleRequest,
    ) -> Result<Schedule, ApiRequestError<ErrResp>> {
        let path = format!("channels/{channel_id}/schedules/{schedule_id}");
        self.request
            .request(&path, Method::PATCH, None, Some(body))
            .await
    }
}
