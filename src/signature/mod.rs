mod error;
mod signing;

pub use error::{Result, SignatureError};
pub use signing::{public_key_from_bot_secret, sign_webhook_validation};
