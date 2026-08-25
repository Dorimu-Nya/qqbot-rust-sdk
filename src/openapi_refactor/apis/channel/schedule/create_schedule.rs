use http::Method;

use super::super::super::super::client::{ApiRequestError, ApiRequestExt};
use super::super::super::super::models::err_resp::ErrResp;
use super::models::schedule::Schedule;
use super::models::upsert_schedule_request::UpsertScheduleRequest;
use super::ScheduleApi;

impl ScheduleApi {
    pub async fn create_schedule(
        &self,
        channel_id: &str,
        body: &UpsertScheduleRequest,
    ) -> Result<Schedule, ApiRequestError<ErrResp>> {
        let path = format!("channels/{channel_id}/schedules");
        self.request
            .request(&path, Method::POST, None, Some(body))
            .await
    }
}
