use super::{Result, SignatureError};
use ed25519_dalek::{Signer, SigningKey};

/// 计算 webhook 签名后的字符串
///
/// # Parameters
///
/// - `bot_secret`: 机器人密钥
/// - `event_ts`: 计算签名使用时间戳
/// - `plain_token`: 需要计算签名的字符串
///
/// # Returns
///
/// 返回计算后的签名
pub fn sign_webhook_validation(
    bot_secret: &str,
    event_ts: &str,
    plain_token: &str,
) -> Result<String> {
    let signing_key = signing_key_from_bot_secret(bot_secret)?;
    let mut payload = Vec::with_capacity(event_ts.len() + plain_token.len());
    payload.extend_from_slice(event_ts.as_bytes());
    payload.extend_from_slice(plain_token.as_bytes());
    let signature = signing_key.sign(&payload);
    Ok(hex::encode(signature.to_bytes()))
}

/// 使用 Bot Secret 推导 Ed25519 私钥种子。
fn signing_key_from_bot_secret(bot_secret: &str) -> Result<SigningKey> {
    if bot_secret.is_empty() {
        return Err(SignatureError::EmptyBotSecret);
    }
    // 与官方实现保持兼容：通过重复 secret 填充到 32 字节种子。
    let mut seed = Vec::new();
    while seed.len() < 32 {
        seed.extend_from_slice(bot_secret.as_bytes());
    }
    seed.truncate(32);
    let mut seed_arr = [0u8; 32];
    seed_arr.copy_from_slice(&seed);
    Ok(SigningKey::from_bytes(&seed_arr))
}
