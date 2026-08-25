use reqwest::Method;

use super::super::super::super::client::{ApiRequestError, ApiRequestExt};
use super::super::super::super::models::err_resp::ErrResp;
use super::models::schedule::Schedule;
use super::ScheduleApi;

impl ScheduleApi {
    pub async fn list_schedules(
        &self,
        channel_id: &str,
        since: Option<u64>,
    ) -> Result<Vec<Schedule>, ApiRequestError<ErrResp>> {
        let path = format!("channels/{channel_id}/schedules");
        let mut query = Vec::new();
        if let Some(since) = since {
            query.push(("since", since.to_string()));
        }
        self.request
            .request::<serde_json::Value, Vec<Schedule>, ErrResp>(
                &path,
                Method::GET,
                Some(&query),
                None,
            )
            .await
    }
}
