mod key_store;
mod salt;
mod util;
pub mod synchronize;
pub mod asynchronize;

pub use synchronize::Reader as SyncReader;
pub use synchronize::Writer as SyncWriter;
pub use asynchronize::Reader as AsyncReader;
pub use asynchronize::Writer as AsyncWriter;

use std::fmt;
use postgres as pg;

type DatabaseResult<T> = Result<T, Box<dyn std::error::Error>>;

pub enum Error {
    InvalidCharacter(InvalidCharacterError),
    InsertionFail(InsertionFailError),
    DotenvLineParse(String, usize),
    DotenvIo(std::io::Error),
    DotenvEnvVar(std::env::VarError),
    Nonce(NonceError),
    PostgresDb(pg::error::DbError),
    Postgres(pg::error::Error),
    PostgresSqlState(pg::error::SqlState),
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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
