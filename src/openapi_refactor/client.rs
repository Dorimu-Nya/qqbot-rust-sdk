use async_trait::async_trait;
use reqwest::Method;
use serde::de::DeserializeOwned;
use serde::Serialize;

use super::api_request::ApiRequest;

const API_BASE_URL: &str = "https://api.sgroup.qq.com/";

#[derive(Debug)]
pub enum ApiClientError {
    Url(String),
    Middleware(reqwest_middleware::Error),
}

#[derive(Debug)]
pub enum ApiRequestError<E> {
    Client(ApiClientError),
    Serialize(serde_json::Error),
    Http(reqwest::Error),
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
        body: Option<serde_json::Value>,
    ) -> Result<reqwest::Response, ApiClientError> {
        let url = self
            .base_url
            .join(path)
            .map_err(|error| ApiClientError::Url(error.to_string()))?;
        let mut request = self.client.request(method, url);
        if let Some(query) = query {
            request = request.query(query);
        }
        if let Some(body) = body {
            request = request.json(&body);
        }

        request.send().await.map_err(ApiClientError::Middleware)
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
            .map(serde_json::to_value)
            .transpose()
            .map_err(ApiRequestError::Serialize)?;
        let response = self
            .send(path, method, query, body)
            .await
            .map_err(ApiRequestError::Client)?;
        let status = response.status();
        if status.is_success() {
            if matches!(
                status,
                reqwest::StatusCode::NO_CONTENT | reqwest::StatusCode::RESET_CONTENT
            ) {
                return serde_json::from_value(serde_json::Value::Null)
                    .map_err(ApiRequestError::Serialize);
            }

            response
                .json::<ResponseBody>()
                .await
                .map_err(ApiRequestError::Http)
        } else {
            let body = response
                .json::<ErrorBody>()
                .await
                .map_err(ApiRequestError::Http)?;
            Err(ApiRequestError::Response { status, body })
        }
    }
}
