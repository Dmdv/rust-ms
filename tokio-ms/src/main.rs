// Using Tokio runtime

// fn main() {
//     tokio::runtime::Runtime::new().unwrap().block_on(async {
//         println!("Hello world");
//     })
// }

// Here’s how you would normally bootstrap the Tokio runtime.

// #[tokio::main]
// async fn main() {
//     println!("Hello world");
// }

// Tokio tasks

use std::time::Duration;
use tokio::time::sleep;

// #[tokio::main]
// async fn main() {
//     let (v1, v2, v3) = tokio::join!(
//         async {
//             sleep(Duration::from_millis(1500)).await;
//             println!("Value 1 ready");
//             "Value 1"
//         },
//         async {
//             sleep(Duration::from_millis(2800)).await;
//             println!("Value 2 ready");
//             "Value 2"
//         },
//         async {
//             sleep(Duration::from_millis(600)).await;
//             println!("Value 3 ready");
//             "Value 3"
//         },
//     );
//
//     assert_eq!(v1, "Value 1");
//     assert_eq!(v2, "Value 2");
//     assert_eq!(v3, "Value 3");
// }

use std::sync::Arc;
use tokio::sync::mpsc::unbounded_channel;
use tokio::sync::Mutex;

#[derive(Debug)]
enum Message {
    SendWelcomeEmail { to: String },
    DownloadVideo { id: usize },
    GenerateReport,
    Terminate,
}

// https://blog.logrocket.com/asynchronous-i-o-and-async-await-packages-in-rust
// https://tokio.rs/#tk-lib-runtime

#[tokio::main]
async fn main() {
    let (sender, receiver) = unbounded_channel();
    let receiver = Arc::new(Mutex::new(receiver));

    // spawn a few workers

    let size = 5;
    let mut workers = Vec::with_capacity(size);

    for id in 0..size {
        let receiver = Arc::clone(&receiver);
        let worker = tokio::spawn(async move {
            loop {
                let message = receiver
                    .lock()
                    .await
                    .recv()
                    .await
                    .unwrap_or_else(|| Message::Terminate);
                println!("Worker {}: {:?}", id, message);
                match message {
                    Message::Terminate => break,
                    _ => sleep(Duration::from_secs(1 + id as u64)).await,
                }
            }
        });
        workers.push(worker);
    }

    sender.send(Message::DownloadVideo { id: 10 }).unwrap();
    sender.send(Message::GenerateReport).unwrap();
    sender.send(Message::SendWelcomeEmail { to: "hi@example.com".into() }).unwrap();
    sender.send(Message::DownloadVideo { id: 25 }).unwrap();

    for _ in &workers {
        let _ = sender.send(Message::Terminate);
    }
    for worker in workers {
        let _ = worker.await;
    }
}