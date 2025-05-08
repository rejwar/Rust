use async_stream::stream;
use tokio::time::{sleep, Duration};

async fn ExampleStream() {
    let MyStream = stream! {
        for i in 1..5 {
            sleep(Duration::from_secs(1)).await;
            yield i;
        }
    };

    tokio::pin!(MyStream);

    while let Some(Value) = MyStream.next().await {
        println!("Received: {}", Value);
    }
}

#[tokio::main]
async fn main() {
    ExampleStream().await;
}
