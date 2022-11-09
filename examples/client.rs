use anyhow::Result;
use rdqs::{
    client::{Client, ClientError, Redis},
    queue::Queue,
    queue_job,
    worker::Worker,
};
use serde::{Deserialize, Serialize};
use tokio::time::Duration;

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
    let client: Client<Redis> = Client::new(None);
    client
        .connect()
        .await
        .map_err(|e: ClientError| println!("error {:?}", e))
        .unwrap();
    println!("{:?}!", client);

    let queue = Queue::new("foo".to_string(), None, None);

    let mut worker = Worker::new();

    worker.register_queue_job(&queue, on_foo_event);

    Ok(())
}
