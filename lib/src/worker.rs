use std::{collections::HashMap, pin::Pin};

use tokio::task::JoinHandle;

use crate::queue::Queue;

type FutResult = Result<JoinHandle<Result<(), String>>, serde_json::Error>;
pub struct Worker {
    map: HashMap<String, Pin<Box<fn(String) -> FutResult>>>,
}

impl Worker {
    pub fn new() -> Self {
        Worker {
            map: HashMap::new(),
        }
    }

    pub fn register_queue_job(&mut self, queue: &Queue, cb: fn(String) -> FutResult) {
        let mut _cb = Pin::new(Box::new(cb));
        let path = if let Some(ref prefix) = queue.prefix {
            let mut value = prefix.clone();
            value.push('/');
            value.push_str(&queue.name);
            value
        } else {
            let mut value = String::from("rdqs/");
            value.push_str(&queue.name);
            value
        };
        self.map.insert(path, _cb);
    }
    pub fn start(&self) -> Result<(), String> {
        todo!()
    }
}
