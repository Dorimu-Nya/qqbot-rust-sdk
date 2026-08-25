use async_trait::async_trait;
use reqwest::Method;

use super::client::ApiClientError;

#[async_trait]
pub trait ApiRequest: Send + Sync {
    async fn send(
        &self,
        path: &str,
        method: Method,
        query: Option<&[(&str, String)]>,
        body: Option<serde_json::Value>,
    ) -> Result<reqwest::Response, ApiClientError>;
}
