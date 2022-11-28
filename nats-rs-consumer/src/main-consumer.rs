use futures::stream::StreamExt;
use std::{env, str::from_utf8};

// Example 1:
// https://natsbyexample.com/examples/messaging/concurrent/rust
// Example of concurrent message processing
#[tokio::main]
async fn main() -> Result<(), async_nats::Error> {
    let nats_url = env::var("NATS_URL")
        .unwrap_or_else(|_| "nats://localhost:4222".to_string());

    let client = async_nats::connect(nats_url).await?;
    // let subscription = client.subscribe("greet.*".to_string()).await?;
    let subscription = client.queue_subscribe(
        "greet.*".to_string(),
        "demo".to_string())
        .await?;

    subscription
        .for_each_concurrent(25, |message| async move {
            println!(
                "received message: {:?}",
                from_utf8(&message.payload).unwrap()
            )
        })
        .await;

    Ok(())
}