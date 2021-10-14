use crate::Result;
use crate::salt::salt;

const API_KEY_SALT_LEN: usize = 72;
pub struct Storage {
    pub index: Vec<u8>,
    pub text: Vec<u8>,
    pub salt: Vec<u8>,
}

pub fn encode(uid: &str, key: &str) -> Result<Storage> {
    // Ghan's algorithm
    // M = fast_hash(uid)
    // F = slow_hash(M, uid+salt)
    // T = xor(F, key)
    let config = argon2::Config::default();
    let salt = salt(API_KEY_SALT_LEN)?.into_bytes();
    let uid_u8: Vec<u8> = uid.clone().as_bytes().into();
    assert!(uid_u8.len() <= salt.len());

    let mut salted_uid: Vec<u8> = salt.clone();
    salted_uid.splice(0.., uid_u8);

    let index: [u8; 32] = blake3::hash(uid.as_bytes()).into();
    let tmp = argon2::hash_encoded(&index, &salted_uid, &config)?;
    let text: Vec<u8> = tmp
        .as_bytes()
        .iter()
        .zip(key.as_bytes().iter())
        .map(|(&x, &y)| x ^ y)
        .collect();

    assert_eq!(index.len(), 32);
    assert_eq!(text.len(), API_KEY_SALT_LEN);
    assert_eq!(salt.len(), API_KEY_SALT_LEN);

    let index = index.iter().cloned().collect();

    Ok(Storage { index, text, salt })
}

pub fn index(uid: &str) -> blake3::Hash {
    blake3::hash(uid.as_bytes())
}

pub fn decode(s: &Storage, uid: &str) -> Result<String> {
    // Ghan's algorithm
    // M = fast_hash(uid)
    // F = slow_hash(M, uid+salt)
    // key = xor(F, T)
    let config = argon2::Config::default();
    let uid_u8: Vec<u8> = uid.clone().as_bytes().into();
    assert!(uid_u8.len() <= s.salt.len());

    let mut salted_uid: Vec<u8> = s.salt.clone();
    salted_uid.splice(0.., uid_u8);

    let tmp = argon2::hash_encoded(&s.index, &salted_uid, &config)?;
    let api_key: Vec<u8> = tmp
        .as_bytes()
        .iter()
        .zip(s.text.iter())
        .map(|(x, y)| x ^ y)
        .collect();
    let api_key = String::from_utf8(api_key)?;

    Ok(api_key)
}
