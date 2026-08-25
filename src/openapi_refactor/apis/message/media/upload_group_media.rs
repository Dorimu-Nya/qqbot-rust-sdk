use http::Method;
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};

use super::super::super::super::client::{ApiRequestError, ApiRequestExt};
use super::super::super::super::models::err_resp::ErrResp;
use super::MediaApi;

impl MediaApi {
    pub async fn upload_group_media(
        &self,
        group_openid: &str,
        body: &super::models::upload_media_request::UploadMediaRequest,
    ) -> Result<super::models::upload_media_response::UploadMediaResponse, ApiRequestError<ErrResp>>
    {
        let group_openid = utf8_percent_encode(group_openid, NON_ALPHANUMERIC);
        let path = format!("v2/groups/{group_openid}/files");

        self.request
            .request(&path, Method::POST, None, Some(body))
            .await
    }
}
