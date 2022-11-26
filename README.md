## Rust microservices in many flavours

### actix-svc
Actix example

### http-server-simple
Using TcpListener and stream example

### http-server-standard
Using TcpListener and stream example

### Mount
Static files mount to the local folder

### Local time

```rust
use chrono::prelude::*;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

fn main() {
    let local_time: DateTime<Local> = Local::now();
    let time_str = local_time.format("%H:%M:%S%.3f").to_string();
    println!("{}", time_str);

    let start = Instant::now();
    if start.elapsed() < Duration::from_secs(10) {
        // Warming up.
    } else if start.elapsed() < Duration::from_secs(20) {
        if latencies.len() == 0 {
            println!("Recording for 10s...");
        }
        latencies += (now() - then) as u64;
    }
}


fn now() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
        .try_into()
        .unwrap()
}

```

### Command line

```rust
#[tokio::main]
async fn main() {
    let matches = App::new("Round-trip example")
        .version(option_env!("CARGO_PKG_VERSION").unwrap_or(""))
        .about("Measures latency between producer and consumer")
        .arg(
            Arg::with_name("brokers")
                .short("b")
                .long("brokers")
                .help("Broker list in kafka format")
                .takes_value(true)
                .default_value("localhost:9092"),
        )
        .arg(
            Arg::with_name("topic")
                .long("topic")
                .help("topic")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("log-conf")
                .long("log-conf")
                .help("Configure the logging format (example: 'rdkafka=trace')")
                .takes_value(true),
        )
        .get_matches();

    let brokers = matches.value_of("brokers").unwrap();
    let topic = matches.value_of("topic").unwrap().to_owned();
    
    println!("{}", brokers);
    println!("{}", topic);
}
```