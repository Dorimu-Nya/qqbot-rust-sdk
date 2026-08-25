use std::error::Error;

use async_trait::async_trait;
use http::Method;
use reqwest::header::CONTENT_TYPE;

use super::api_request::ApiRequest;

const API_BASE_URL: &str = "https://api.sgroup.qq.com/";

#[derive(Clone)]
pub struct ReqwestClient {
    base_url: reqwest::Url,
    client: reqwest_middleware::ClientWithMiddleware,
}

impl ReqwestClient {
    pub fn new(client: reqwest_middleware::ClientWithMiddleware) -> Self {
        Self {
            base_url: reqwest::Url::parse(API_BASE_URL).expect("API_BASE_URL must be a valid URL"),
            client,
        }
    }
}

#[async_trait]
impl ApiRequest for ReqwestClient {
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
