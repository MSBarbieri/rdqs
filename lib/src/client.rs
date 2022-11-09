use async_trait::async_trait;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClientError {
    ClientOptionsInvalid(String),
    LimitExeeded,
}

pub(crate) trait ClientTypeSettigns {}

#[async_trait]
pub trait ClientType {
    async fn connect(&self) -> Result<(), ClientError> {
        self.valitate_connection_config()?;

        Ok(())
    }
    fn valitate_connection_config(&self) -> Result<(), ClientError>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Client<T: ClientType>(T);

///
/// Redis Config
///

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RedisSettings {
    url: String,
}

impl Default for RedisSettings {
    fn default() -> Self {
        RedisSettings {
            url: "127.0.0.1:5643".to_string(),
        }
    }
}

impl ClientTypeSettigns for RedisSettings {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Redis {
    opts: Box<RedisSettings>,
}

impl Redis {
    fn new(opts: Option<Box<RedisSettings>>) -> Self {
        let opts = opts.unwrap_or_default();
        Redis { opts }
    }
}

impl Default for Redis {
    fn default() -> Self {
        Redis {
            opts: Box::new(RedisSettings::default()),
        }
    }
}

impl ClientType for Redis {
    fn valitate_connection_config(&self) -> Result<(), ClientError> {
        if self.opts.url.is_empty() {
            return Err(ClientError::ClientOptionsInvalid(
                "validation dont exist".to_string(),
            ));
        }

        Ok(())
    }
}

impl Client<Redis> {
    pub fn new(opts: Option<Box<RedisSettings>>) -> Client<Redis> {
        let mut _client = Redis::new(opts);
        Self(_client)
    }

    pub async fn connect(&self) -> Result<(), ClientError> {
        let mut con = self.0.connect().await?;
        Ok(())
    }
}
