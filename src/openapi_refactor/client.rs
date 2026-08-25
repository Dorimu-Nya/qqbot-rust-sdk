use std::error::Error;

use async_trait::async_trait;
use http::Method;
use reqwest::header::CONTENT_TYPE;
use serde::de::DeserializeOwned;
use serde::Serialize;

use super::api_request::ApiRequest;

const API_BASE_URL: &str = "https://api.sgroup.qq.com/";

#[derive(Debug)]
pub enum ApiRequestError<E> {
    Transport(Box<dyn Error + Send + Sync>),
    Serialize(serde_json::Error),
    Deserialize(serde_json::Error),
    Response {
        status: reqwest::StatusCode,
        body: E,
    },
}

#[derive(Clone)]
pub struct ApiClient {
    base_url: reqwest::Url,
    client: reqwest_middleware::ClientWithMiddleware,
}

impl ApiClient {
    pub fn new(client: reqwest_middleware::ClientWithMiddleware) -> Self {
        Self {
            base_url: reqwest::Url::parse(API_BASE_URL).expect("API_BASE_URL must be a valid URL"),
            client,
        }
    }
}

pub trait ApiRequestExt {
    async fn request<RequestBody, ResponseBody, ErrorBody>(
        &self,
        path: &str,
        method: Method,
        query: Option<&[(&str, String)]>,
        body: Option<&RequestBody>,
    ) -> Result<ResponseBody, ApiRequestError<ErrorBody>>
    where
        RequestBody: Serialize + ?Sized,
        ResponseBody: DeserializeOwned,
        ErrorBody: DeserializeOwned;
}

#[async_trait]
impl ApiRequest for ApiClient {
    async fn send(
        &self,
        path: &str,
        method: Method,
        query: Option<&[(&str, String)]>,
        body: Option<Vec<u8>>,
    ) -> Result<(http::StatusCode, Vec<u8>), Box<dyn Error + Send + Sync>> {
        let url = self
            .base_url
            .join(path)
            .map_err(|error| -> Box<dyn Error + Send + Sync> { Box::new(error) })?;
        let mut request = self.client.request(method, url);
        if let Some(query) = query {
            request = request.query(query);
        }
        if let Some(body) = body {
            request = request.header(CONTENT_TYPE, "application/json").body(body);
        }

        let response = request
            .send()
            .await
            .map_err(|error| -> Box<dyn Error + Send + Sync> { Box::new(error) })?;
        let status = response.status();
        let body = response
            .bytes()
            .await
            .map_err(|error| -> Box<dyn Error + Send + Sync> { Box::new(error) })?
            .to_vec();

        Ok((status, body))
    }
}

impl<T> ApiRequestExt for T
where
    T: ApiRequest + ?Sized,
{
    async fn request<RequestBody, ResponseBody, ErrorBody>(
        &self,
        path: &str,
        method: Method,
        query: Option<&[(&str, String)]>,
        body: Option<&RequestBody>,
    ) -> Result<ResponseBody, ApiRequestError<ErrorBody>>
    where
        RequestBody: Serialize + ?Sized,
        ResponseBody: DeserializeOwned,
        ErrorBody: DeserializeOwned,
    {
        let body = body
            .map(serde_json::to_vec)
            .transpose()
            .map_err(ApiRequestError::Serialize)?;
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
