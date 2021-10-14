use crate::key_store::{decode, encode, Storage};
use crate::util::*;
use crate::Result;
use crate::{InsertionFailError, InvalidCharacterError};
use deadpool_postgres as dp;
use pg::NoTls;
use tokio_postgres as pg;

pub struct Writer {
    pool: dp::Pool,
}

pub struct Reader {
    pool: dp::Pool,
}

impl Writer {
    /// Create a new client of database writer.
    /// Need file `.env` contains 4 variables, password is optional.
    /// - `GW2DB_DBNAME` - The name of database.
    /// - `GW2DB_HOST` - The host url of database.
    /// - `GW2DB_WRITER` - The username of database, has permisssion of `INSERT`.
    /// - `GW2DB_WRITER_PW` - The password of the user.
    pub fn new() -> Result<Self> {
        dotenv::dotenv()?;
        let dbname = dotenv::var("GW2DB_DBNAME")?;
        let host = dotenv::var("GW2DB_HOST")?;
        let user = dotenv::var("GW2DB_WRITER")?;
        let mut conf = dp::Config::new();
        conf.dbname = Some(dbname);
        conf.host = Some(host);
        conf.user = Some(user);
        conf.password = dotenv::var("GW2DB_WRITER_PW").ok();
        let pool = conf.create_pool(NoTls)?;

        Ok(Self { pool })
    }

    /// Store api key into database, with help of uid.
    /// `uid` is restricted in alphabets, numbers, and `-`;
    /// `api_key` is also in same restriction.
    pub async fn store(&self, uid: &str, api_key: &str) -> Result<()> {
        match (
            uid.chars().all(is_valid_uid),
            api_key.chars().all(is_valid_key),
        ) {
            (true, true) => (),
            (false, _) => return Err(InvalidCharacterError::from(uid))?,
            (_, false) => return Err(InvalidCharacterError::from(api_key))?,
        }

        let s: Storage = encode(uid, api_key)?;
        let client = self.pool.get().await?;
        let statement = client
            .prepare("INSERT INTO key VALUES ($1, $2, $3)")
            .await?;
        let inserted = client
            .execute(&statement, &[&s.index, &s.text, &s.salt])
            .await?;

        match inserted {
            1 => Ok(()),
            0 => Err(InsertionFailError)?,
            _ => panic!("inserted more than 1 item"),
        }
    }
}

impl Reader {
    /// Create a new client of reader.
    /// Need file `.env` contains 4 variables, password is optional.
    /// - `GW2DB_DBNAME` - The name of database.
    /// - `GW2DB_HOST` - The host url of database.
    /// - `GW2DB_READER` - The username of database, has permission of `SELECT`.
    /// - `GW2DB_READER_PW` - The password of the user.
    pub fn new() -> Result<Reader> {
        dotenv::dotenv()?;
        let dbname = dotenv::var("GW2DB_DBNAME")?;
        let host = dotenv::var("GW2DB_HOST")?;
        let user = dotenv::var("GW2DB_READER")?;
        let mut conf = dp::Config::new();
        conf.dbname = Some(dbname);
        conf.host = Some(host);
        conf.user = Some(user);
        conf.password = dotenv::var("GW2DB_WRITER_PW").ok();
        let pool = conf.create_pool(NoTls)?;

        Ok(Self { pool })
    }

    /// Retrive api key from database, with help of uid.
    /// `uid` is restricted in alphabets, numbers, and `-`.
    pub async fn api(&self, uid: &str) -> Result<String> {
        if let false = uid.chars().all(is_valid_uid) {
            return Err(InvalidCharacterError::from(uid))?;
        }
        let tmp: [u8; 32] = crate::key_store::index(uid).into();
        let index: Vec<u8> = tmp.iter().cloned().collect();

        let client = self.pool.get().await?;
        let statement = client
            .prepare("SELECT secret, salt FROM key WHERE index = $1")
            .await?;
        let row = client.query_one(&statement, &[&index]).await?;

        let text: Vec<u8> = row.get(0);
        let salt: Vec<u8> = row.get(1);

        let s = Storage { index, text, salt };

        decode(&s, uid)
    }
}
