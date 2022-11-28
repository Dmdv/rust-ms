use futures::stream::StreamExt;
use std::{env, str::from_utf8};
use async_nats::Message;

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
            tokio::spawn({
                async move {
                    process_message(&message).await;
                }
            });
        })
        .await;

    Ok(())
}

async fn process_message(m: &Message) -> Result<(), async_nats::Error> {
    Ok::<(), async_nats::Error>(())
}