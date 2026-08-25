use std::error::Error;

use async_trait::async_trait;
use http::{Method, StatusCode};

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
