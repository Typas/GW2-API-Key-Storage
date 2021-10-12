pub fn is_valid_uid(uid: char) -> bool {
    matches!(uid, '0'..='9' | 'A'..='Z' | 'a'..='z' | '-')
}

pub fn is_valid_key(key: char) -> bool {
    is_valid_uid(key)
}
