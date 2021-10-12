use crate::key_store::{decode, encode, Storage};
use crate::util::*;
use crate::DatabaseResult;
use crate::{InsertionFailError, InvalidCharacterError};
use pg::NoTls;
use postgres as pg;

pub struct Writer {
    conf: pg::Config,
}

pub struct Reader {
    conf: pg::Config,
}

impl Writer {
    /// Create a new client of database writer.
    /// Need file `.env` contains 4 variables, password is optional.
    /// - `GW2DB_DBNAME` - The name of database.
    /// - `GW2DB_HOST` - The host url of database.
    /// - `GW2DB_WRITER` - The username of database, has permisssion of `INSERT`.
    /// - `GW2DB_WRITER_PW` - The password of the user.
    pub fn new() -> DatabaseResult<Writer> {
        dotenv::dotenv()?;
        let dbname = dotenv::var("GW2DB_DBNAME")?;
        let host = dotenv::var("GW2DB_HOST")?;
        let user = dotenv::var("GW2DB_WRITER")?;
        let mut conf = pg::Config::new();
        conf.dbname(&dbname);
        conf.host(&host);
        conf.user(&user);
        if let Ok(pw) = dotenv::var("GW2DB_WRITER_PW") {
            conf.password(&pw);
        }

        Ok(Writer { conf })
    }

    /// Store api key into database, with help of uid.
    /// `uid` is restricted in alphabets, numbers, and `-`;
    /// `api_key` is also in same restriction.
    pub fn store(&mut self, uid: &str, api_key: &str) -> DatabaseResult<()> {
        match (
            uid.chars().all(is_valid_uid),
            api_key.chars().all(is_valid_key),
        ) {
            (true, true) => (),
            (false, _) => {
                return Err(Box::new(InvalidCharacterError {
                    message: uid.to_string(),
                }))
            }
            (_, false) => {
                return Err(Box::new(InvalidCharacterError {
                    message: api_key.to_string(),
                }))
            }
        }

        let s: Storage = encode(uid, api_key)?;
        let mut client = self.conf.connect(NoTls)?;

        let inserted = client.execute(
            "INSERT INTO key VALUES ($1, $2, $3)",
            &[&s.index, &s.text, &s.salt],
        )?;

        client.close()?;

        match inserted {
            1 => Ok(()),
            0 => Err(Box::new(InsertionFailError)),
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
    pub fn new() -> DatabaseResult<Reader> {
        dotenv::dotenv()?;
        let dbname = dotenv::var("GW2DB_DBNAME")?;
        let host = dotenv::var("GW2DB_HOST")?;
        let user = dotenv::var("GW2DB_READER")?;
        let mut conf = pg::Config::new();
        conf.dbname(&dbname);
        conf.host(&host);
        conf.user(&user);
        if let Ok(pw) = dotenv::var("GW2DB_READER_PW") {
            conf.password(&pw);
        }
        Ok(Reader { conf })
    }

    /// Retrive api key from database, with help of uid.
    /// `uid` is restricted in alphabets, numbers, and `-`.
    pub fn api(&mut self, uid: &str) -> DatabaseResult<String> {
        if let false = uid.chars().all(is_valid_uid) {
            return Err(Box::new(InvalidCharacterError {
                message: uid.to_string(),
            }));
        }

        let mut client = self.conf.connect(NoTls)?;

        let tmp: [u8; 32] = crate::key_store::index(uid).into();
        let index: Vec<u8> = tmp.iter().cloned().collect();

        let row = client.query_one("SELECT secret, salt FROM key WHERE index = $1", &[&index])?;

        let text: Vec<u8> = row.get(0);
        let salt: Vec<u8> = row.get(1);

        let s = Storage { index, text, salt };

        client.close()?;

        decode(&s)
    }
}
