use std::time::Duration;
use tokio::time::sleep;

async fn AsyncFunction() {
    println!("Start...");
    sleep(Duration::from_secs(2)).await;
    println!("Completed after 2 seconds.");
}

#[tokio::main]
async fn main() {
    AsyncFunction().await;
}
