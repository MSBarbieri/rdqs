use std::time::Duration;

use anyhow::Result;
use rdqs::{queue::Queue, queue_job, worker::Worker};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Foo {
    bar: u32,
    baz: u32,
}

#[queue_job]
async fn on_foo_event(msg: Foo) -> Result<(), String> {
    println!("{:?} vai krl", msg);
    tokio::time::sleep(Duration::from_millis(100)).await;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut worker = Worker::new();

    let queue = Queue::new("base".to_string(), None, None);
    worker.register_queue_job(&queue, on_foo_event);

    worker.start();
    Ok(())
}
