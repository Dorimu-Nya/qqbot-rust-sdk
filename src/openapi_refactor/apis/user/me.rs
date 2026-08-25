use reqwest::Method;

use super::super::super::client::{ApiRequestError, ApiRequestExt};
use super::super::super::models::{err_resp::ErrResp, user::User};
use super::UserApis;

impl UserApis {
    pub async fn me(&self) -> Result<User, ApiRequestError<ErrResp>> {
        self.request
            .request::<serde_json::Value, User, ErrResp>("users/@me", Method::GET, None, None)
            .await
    }
}
