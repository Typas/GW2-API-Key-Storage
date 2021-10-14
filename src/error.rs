use deadpool_postgres as dppg;
use postgres as pg;
use std::fmt;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    InvalidCharacter(InvalidCharacterError),
    InsertionFail(InsertionFailError),
    Dotenv(dotenv::Error),
    Nonce(NonceError),
    Postgres(pg::Error),
    Argon2(argon2::Error),
    Utf8(std::string::FromUtf8Error),
    DeadpoolPostgresPool(dppg::PoolError),
    DeadpoolPostgresConfig(dppg::config::ConfigError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::InvalidCharacter(invalid_char_error) => write!(f, "{}", invalid_char_error),
            Error::InsertionFail(insert_fail_error) => write!(f, "{}", insert_fail_error),
            Error::Dotenv(dotenv_error) => write!(f, "{}", dotenv_error),
            Error::Nonce(nonce_error) => write!(f, "{}", nonce_error),
            Error::Postgres(pg_error) => write!(f, "{}", pg_error),
            Error::Argon2(argon2_error) => write!(f, "{}", argon2_error),
            Error::Utf8(utf8_error) => write!(f, "{}", utf8_error),
            Error::DeadpoolPostgresPool(dp_pool_error) => write!(f, "{}", dp_pool_error),
            Error::DeadpoolPostgresConfig(dp_config_error) => write!(f, "{}", dp_config_error),
        }
    }
}

impl std::error::Error for Error {}

impl From<InvalidCharacterError> for Error {
    fn from(err: InvalidCharacterError) -> Self {
        Self::InvalidCharacter(err)
    }
}

impl From<InsertionFailError> for Error {
    fn from(err: InsertionFailError) -> Self {
        Self::InsertionFail(err)
    }
}

impl From<dotenv::Error> for Error {
    fn from(err: dotenv::Error) -> Self {
        Self::Dotenv(err)
    }
}

impl From<NonceError> for Error {
    fn from(err: NonceError) -> Self {
        Self::Nonce(err)
    }
}

impl From<pg::Error> for Error {
    fn from(err: pg::Error) -> Self {
        Self::Postgres(err)
    }
}

impl From<argon2::Error> for Error {
    fn from(err: argon2::Error) -> Self {
        Self::Argon2(err)
    }
}

impl From<std::string::FromUtf8Error> for Error {
    fn from(err: std::string::FromUtf8Error) -> Self {
        Self::Utf8(err)
    }
}

impl From<dppg::PoolError> for Error {
    fn from(err: dppg::PoolError) -> Self {
        Self::DeadpoolPostgresPool(err)
    }
}

impl From<dppg::config::ConfigError> for Error {
    fn from(err: dppg::config::ConfigError) -> Self {
        Self::DeadpoolPostgresConfig(err)
    }
}

#[derive(Debug)]
pub struct InsertionFailError;

impl fmt::Display for InsertionFailError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Failed to insert data")
    }
}

impl std::error::Error for InsertionFailError {}

#[derive(Debug)]
pub struct NonceError {
    message: String,
}

impl fmt::Display for NonceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error of getting nonce: {}", self.message)
    }
}

impl std::error::Error for NonceError {}

impl From<&str> for NonceError {
    fn from(s: &str) -> Self {
        Self {
            message: s.to_string(),
        }
    }
}

impl From<String> for NonceError {
    fn from(s: String) -> Self {
        Self { message: s }
    }
}

#[derive(Debug)]
pub struct InvalidCharacterError {
    message: String,
}

impl fmt::Display for InvalidCharacterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Contains invalid character: {}", self.message)
    }
}

impl std::error::Error for InvalidCharacterError {}

impl From<&str> for InvalidCharacterError {
    fn from(s: &str) -> Self {
        Self {
            message: s.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
