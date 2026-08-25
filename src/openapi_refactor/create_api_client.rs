use std::sync::Arc;

use reqwest_middleware::ClientBuilder;

use super::api::QQApiClient;
use super::client::ApiClient;
use super::middleware::create_token_middleware;

pub async fn create_qq_api_client(
    app_id: impl Into<String>,
    app_secret: impl Into<String>,
) -> Result<QQApiClient, reqwest_middleware::Error> {
    let middleware = create_token_middleware(app_id, app_secret).await?;
    let client = ClientBuilder::new(reqwest::Client::new())
        .with(middleware)
        .build();

    Ok(QQApiClient {
        request: Arc::new(ApiClient::new(client)),
    })
}
