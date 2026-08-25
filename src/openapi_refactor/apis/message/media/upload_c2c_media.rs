use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use reqwest::Method;

use super::super::super::super::client::{ApiRequestError, ApiRequestExt};
use super::super::super::super::models::err_resp::ErrResp;
use super::MediaApi;

impl MediaApi {
    pub async fn upload_c2c_media(
        &self,
        openid: &str,
        body: &super::models::upload_media_request::UploadMediaRequest,
    ) -> Result<super::models::upload_media_response::UploadMediaResponse, ApiRequestError<ErrResp>>
    {
        let openid = utf8_percent_encode(openid, NON_ALPHANUMERIC);
        let path = format!("v2/users/{openid}/files");

        self.request
            .request(&path, Method::POST, None, Some(body))
            .await
    }
}
