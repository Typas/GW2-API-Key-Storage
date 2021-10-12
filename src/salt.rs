use textnonce::TextNonce;
use crate::DatabaseResult;

use super::NonceError;

/// returns a salt if successfully produced
pub fn salt(n: usize) -> DatabaseResult<String> {
    match TextNonce::sized_urlsafe(n) {
        Ok(nonce) => Ok(nonce.into_string()),
        Err(why) => Err(Box::new(NonceError {
            message: why,
        })),
    }
}
