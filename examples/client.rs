use anyhow::Result;
use rdqs::{client::Client, connection::Redis, queue::Queue};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Foo {
    bar: u32,
    baz: u32,
}

#[tokio::main]
async fn main() -> Result<()> {
    let client: Client<Redis> = Client::new(None);
    let con = client.connect().await.unwrap();
    println!("{:?}!", client);
    println!("{:?}!", con);

    let queue = Queue::new("foo".to_string(), None, None);

    let message = Foo { bar: 10, baz: 10 };
    queue
        .publish_job(con, message, serde_json::Value::Null)
        .await
        .map_err(|_e| {})?;
    Ok(())
}
