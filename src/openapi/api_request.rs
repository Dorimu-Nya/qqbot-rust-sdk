use std::error::Error;
use std::future::Future;

use async_trait::async_trait;
use http::{Method, StatusCode};
use serde::de::DeserializeOwned;
use serde::Serialize;

#[derive(Debug)]
pub enum ApiRequestError<E> {
    Transport(Box<dyn Error + Send + Sync>),
    Serialize(serde_json::Error),
    Deserialize(serde_json::Error),
    Response { status: StatusCode, body: E },
}

#[async_trait]
pub trait ApiRequest: Send + Sync {
    async fn send(
        &self,
        path: &str,
        method: Method,
        query: Option<&[(&str, String)]>,
        body: Option<Vec<u8>>,
    ) -> Result<(StatusCode, Vec<u8>), Box<dyn Error + Send + Sync>>;
}

pub trait ApiRequestExt {
    fn request<'a, RequestBody, ResponseBody, ErrorBody>(
        &'a self,
        path: &'a str,
        method: Method,
        query: Option<&'a [(&str, String)]>,
        body: Option<&RequestBody>,
    ) -> impl Future<Output = Result<ResponseBody, ApiRequestError<ErrorBody>>> + Send + 'a
    where
        RequestBody: Serialize + ?Sized,
        ResponseBody: DeserializeOwned,
        ErrorBody: DeserializeOwned;
}

impl<T> ApiRequestExt for T
where
    T: ApiRequest + ?Sized,
{
    fn request<'a, RequestBody, ResponseBody, ErrorBody>(
        &'a self,
        path: &'a str,
        method: Method,
        query: Option<&'a [(&str, String)]>,
        body: Option<&RequestBody>,
    ) -> impl Future<Output = Result<ResponseBody, ApiRequestError<ErrorBody>>> + Send + 'a
    where
        RequestBody: Serialize + ?Sized,
        ResponseBody: DeserializeOwned,
        ErrorBody: DeserializeOwned,
    {
        let body = body.map(serde_json::to_vec).transpose();

        async move {
            let body = body.map_err(ApiRequestError::Serialize)?;
            let (status, response_body) = self
                .send(path, method, query, body)
                .await
                .map_err(ApiRequestError::Transport)?;
            if status.is_success() {
                let body = if response_body.is_empty() {
                    b"null".as_slice()
                } else {
                    response_body.as_slice()
                };
                serde_json::from_slice(body).map_err(ApiRequestError::Deserialize)
            } else {
                let body = serde_json::from_slice::<ErrorBody>(&response_body)
                    .map_err(ApiRequestError::Deserialize)?;
                Err(ApiRequestError::Response { status, body })
            }
        }
    }
}
