use reqwest::Method;

use super::super::super::super::client::{ApiRequestError, ApiRequestExt};
use super::super::super::super::models::err_resp::ErrResp;
use super::models::message_setting::MessageSetting;
use super::SettingsApi;

impl SettingsApi {
    pub async fn get_message_setting(
        &self,
        guild_id: &str,
    ) -> Result<MessageSetting, ApiRequestError<ErrResp>> {
        let path = format!("guilds/{guild_id}/message/setting");
        self.request
            .request::<serde_json::Value, MessageSetting, ErrResp>(&path, Method::GET, None, None)
            .await
    }
}
