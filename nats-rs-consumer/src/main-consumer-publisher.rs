use futures::stream::StreamExt;
use rand::Rng;
use std::{env, str::from_utf8, time::Duration};

// Example 1:
// https://natsbyexample.com/examples/messaging/concurrent/rust
// Example of concurrent message processing
#[tokio::main]
async fn main() -> Result<(), async_nats::Error> {
    let nats_url = env::var("NATS_URL")
        .unwrap_or_else(|_| "nats://localhost:4222".to_string());

    let client = async_nats::connect(nats_url).await?;

    let subscription = client.subscribe("greet.*".to_string()).await?
        .take(50)
        ;

    for i in 0..50 {
        client
            .publish("greet.joe".to_string(), format!("hello {}", i).into())
            .await?;
    }

    subscription
        .for_each_concurrent(25, |message| async move {
            let num = rand::thread_rng().gen_range(0..500);
            tokio::time::sleep(Duration::from_millis(num)).await;

            println!(
                "received message: {:?}",
                from_utf8(&message.payload).unwrap()
            )
        })
        .await;

    Ok(())
}