use thiserror::Error;

pub type Result<T> = std::result::Result<T, SignatureError>;

/// Webhook 签名相关错误。
#[derive(Debug, Error)]
pub enum SignatureError {
    #[error("bot secret must not be empty")]
    EmptyBotSecret,
}
