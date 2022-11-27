use futures::StreamExt;
use std::{env, str::from_utf8, time::Duration};

#[tokio::main]
async fn main() -> Result<(), async_nats::Error> {
    let nats_url = env::var("NATS_URL")
        .unwrap_or_else(|_| "nats://localhost:4222".to_string());

    let client = async_nats::connect(nats_url).await?;

    let mut requests = client.subscribe("greet.*".into()).await.unwrap();

    tokio::spawn({
        let client = client.clone();
        async move {
            while let Some(request) = requests.next().await {
                if let Some(reply) = request.reply {
                    let name = &request.subject[6..];
                    client
                        .publish(reply, format!("hello, {}", name).into())
                        .await?;
                }
            }
            Ok::<(), async_nats::Error>(())
        }
    });

    let response = client.request("greet.sue".into(), "".into()).await?;
    println!("got a response: {:?}", from_utf8(&response.payload)?);

    let response = client.request("greet.john".into(), "".into()).await?;
    println!("got a response: {:?}", from_utf8(&response.payload)?);


    let response = tokio::time::timeout(
        Duration::from_millis(500),
        client.request("greet.bob".into(), "".into()),
    ).await??;

    println!("got a response: {:?}", from_utf8(&response.payload)?);

    Ok(())
}