use crate::DatabaseResult;

use super::salt::salt;

const API_KEY_SALT_LEN: usize = 72;
pub struct Storage {
    pub index: Vec<u8>,
    pub text: Vec<u8>,
    pub salt: Vec<u8>,
}

pub fn encode(uid: &str, key: &str) -> DatabaseResult<Storage> {
    let config = argon2::Config::default();
    let salt = salt(API_KEY_SALT_LEN)?;

    let index: [u8; 32] = blake3::hash(uid.as_bytes()).into();

    let tmp = match argon2::hash_encoded(&index, salt.as_bytes(), &config) {
        Ok(result) => Ok(result),
        Err(why) => Err(Box::new(why.to_owned())),
    }?;

    let text: Vec<u8> = tmp
        .as_bytes()
        .iter()
        .zip(key.as_bytes().iter())
        .map(|(&x, &y)| x ^ y)
        .collect();

    assert_eq!(index.len(), 32);
    assert_eq!(text.len(), API_KEY_SALT_LEN);
    assert_eq!(salt.len(), API_KEY_SALT_LEN);

    let salt = salt.into_bytes();
    let index = index.iter().cloned().collect();

    Ok(Storage { index, text, salt })
}

pub fn index(uid: &str) -> blake3::Hash {
    blake3::hash(uid.as_bytes())
}

pub fn decode(s: &Storage) -> DatabaseResult<String> {
    let config = argon2::Config::default();

    let tmp = match argon2::hash_encoded(&s.index, &s.salt, &config) {
        Ok(result) => Ok(result),
        Err(why) => Err(Box::new(why.to_owned())),
    }?;

    let api_key: Vec<u8> = tmp
        .as_bytes()
        .iter()
        .zip(s.text.iter())
        .map(|(x, y)| x ^ y)
        .collect();

    Ok(String::from_utf8(api_key)?)
}
