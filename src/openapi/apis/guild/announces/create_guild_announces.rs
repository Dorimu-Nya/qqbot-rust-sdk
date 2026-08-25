use http::Method;

use super::super::super::super::api_request::{ApiRequestError, ApiRequestExt};
use super::super::super::super::models::err_resp::ErrResp;
use super::models::announces::Announces;
use super::models::create_announces_request::CreateAnnouncesRequest;
use super::AnnouncesApi;

impl AnnouncesApi {
    pub async fn create_guild_announces(
        &self,
        guild_id: &str,
        body: &CreateAnnouncesRequest,
    ) -> Result<Announces, ApiRequestError<ErrResp>> {
        let path = format!("guilds/{guild_id}/announces");
        self.request
            .request(&path, Method::POST, None, Some(body))
            .await
    }
}
