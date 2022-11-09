use async_trait::async_trait;
use std::fmt::Debug as DebugTrait;

#[derive(Clone, Debug)]
pub enum ConnectionError {
    TimeLimitExceeded,
    RetryLimitExceeded,
    ConnectionOptionsInvalid(String),
}

pub(crate) trait ConnectionSettings {}

pub trait Connection: DebugTrait {}

#[async_trait]
pub trait ConnectionType {
    async fn connect(&self) -> Result<Box<dyn Connection>, ConnectionError>;
    fn valitate_connection_config(&self) -> Result<(), ConnectionError>;
}

#[derive(Debug, Clone)]
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

impl ConnectionSettings for RedisSettings {}

impl Default for Redis {
    fn default() -> Self {
        Redis {
            opts: Box::new(RedisSettings::default()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Redis {
    opts: Box<RedisSettings>,
}

impl Redis {
    pub fn new(opts: Option<Box<RedisSettings>>) -> Self {
        let opts = opts.unwrap_or_default();
        Redis { opts }
    }
}

#[derive(Clone, Debug)]
pub struct RedisConnection;

impl Connection for RedisConnection {}

#[async_trait]
impl ConnectionType for Redis {
    fn valitate_connection_config(&self) -> Result<(), ConnectionError> {
        if self.opts.url.is_empty() {
            return Err(ConnectionError::ConnectionOptionsInvalid(
                "validation dont exist".to_string(),
            ));
        }

        Ok(())
    }

    async fn connect(&self) -> Result<Box<dyn Connection>, ConnectionError> {
        Ok(Box::new(RedisConnection))
    }
}
