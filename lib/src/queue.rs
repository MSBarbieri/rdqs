use std::marker::PhantomData;

use serde::Serialize;

use crate::connection::Connection;
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum QueueEvents {
    Published,
    Enqueued,
    Aborted,
}

pub enum QueueErrors {
    PublishFailed,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Queue {
    pub(crate) name: String,
    pub(crate) prefix: Option<String>,
    opts: Option<String>,
}

impl Queue {
    pub fn new(name: String, prefix: Option<String>, opts: Option<String>) -> Self {
        Queue { name, prefix, opts }
    }

    pub async fn publish_job<T: Serialize>(
        &self,
        con: Box<dyn Connection>,
        message: T,
        opts: serde_json::Value,
    ) -> Result<QueueEvents, QueueErrors> {
        // serde_json::to_string(message);
        todo!()
    }
}
