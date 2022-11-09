use crate::connection::{Connection, ConnectionError, ConnectionType, Redis, RedisSettings};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClientError {
    ClientOptionsInvalid(String),
    LimitExeeded,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Client<T: ConnectionType>(T);

impl Client<Redis> {
    pub fn new(opts: Option<Box<RedisSettings>>) -> Client<Redis> {
        let mut _client = Redis::new(opts);
        Self(_client)
    }

    pub async fn connect(&self) -> Result<Box<dyn Connection>, ConnectionError> {
        self.0.connect().await
    }
}
