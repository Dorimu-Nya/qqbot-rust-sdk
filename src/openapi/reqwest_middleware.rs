use std::time::{Duration, Instant};

use async_trait::async_trait;
use http::Extensions;
use reqwest::header::{HeaderValue, AUTHORIZATION};
use reqwest::Request;
use reqwest_middleware::{Middleware, Next, Result};
use serde::Deserialize;
use tokio::sync::Mutex;

const TOKEN_URL: &str = "https://bots.qq.com/app/getAppAccessToken";
const REFRESH_MARGIN: Duration = Duration::from_secs(60);

/// 为 OpenAPI 请求维护并写入机器人 token 的中间件。
pub struct TokenMiddleware {
    app_id: String,
    secret: String,
    token: Mutex<Token>,
}

#[derive(Deserialize)]
struct Token {
    access_token: String,
    expires_in: u64,
    #[serde(skip, default = "Instant::now")]
    expires_at: Instant,
}

/// 创建使用应用凭据自动获取 token 的中间件。
pub async fn create_token_middleware(
    app_id: impl Into<String>,
    secret: impl Into<String>,
) -> Result<TokenMiddleware> {
    let app_id = app_id.into();
    let secret = secret.into();
    let token = fetch_token(&app_id, &secret).await?;

    Ok(TokenMiddleware {
        app_id,
        secret,
        token: Mutex::new(token),
    })
}

impl TokenMiddleware {
    async fn token(&self) -> Result<String> {
        let mut cached = self.token.lock().await;
        let now = Instant::now();
        if cached.expires_at > now + REFRESH_MARGIN {
            return Ok(cached.access_token.clone());
        }

        let token = fetch_token(&self.app_id, &self.secret).await?;
        let value = token.access_token.clone();
        *cached = token;
        Ok(value)
    }
}

async fn fetch_token(app_id: &str, secret: &str) -> Result<Token> {
    let response = reqwest::Client::new()
        .post(TOKEN_URL)
        .json(&serde_json::json!({
            "appId": app_id,
            "clientSecret": secret,
        }))
        .send()
        .await?
        .error_for_status()?;
    let mut token: Token = response.json().await?;
    token.expires_at = Instant::now() + Duration::from_secs(token.expires_in);
    Ok(token)
}

#[async_trait]
impl Middleware for TokenMiddleware {
    async fn handle(
        &self,
        mut request: Request,
        extensions: &mut Extensions,
        next: Next<'_>,
    ) -> Result<reqwest::Response> {
        let token = self.token().await?;
        let authorization = HeaderValue::from_str(&format!("QQBot {token}"))
            .map_err(reqwest_middleware::Error::middleware)?;
        request.headers_mut().insert(AUTHORIZATION, authorization);
        next.run(request, extensions).await
    }
}
