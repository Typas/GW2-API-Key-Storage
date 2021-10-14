use crate::{Result, NonceError};
use textnonce::TextNonce;

/// returns a salt if successfully produced
pub fn salt(n: usize) -> Result<String> {
    match TextNonce::sized_urlsafe(n) {
        Ok(nonce) => Ok(nonce.into_string()),
        Err(why) => Err(NonceError::from(why))?,
    }
}
