use http::Method;

use super::super::super::super::client::{ApiRequestError, ApiRequestExt};
use super::super::super::super::models::err_resp::ErrResp;
use super::AnnouncesApi;

impl AnnouncesApi {
    pub async fn delete_guild_announces(
        &self,
        guild_id: &str,
        message_id: &str,
    ) -> Result<serde_json::Value, ApiRequestError<ErrResp>> {
        let path = format!("guilds/{guild_id}/announces/{message_id}");
        self.request
            .request::<serde_json::Value, serde_json::Value, ErrResp>(
                &path,
                Method::DELETE,
                None,
                None,
            )
            .await
    }
}
