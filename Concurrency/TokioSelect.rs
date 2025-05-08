use tokio::{time, select};
use std::time::Duration;

async fn TaskOne() {
    time::sleep(Duration::from_secs(3)).await;
    println!("Task One Completed");
}

async fn TaskTwo() {
    time::sleep(Duration::from_secs(2)).await;
    println!("Task Two Completed");
}

#[tokio::main]
async fn main() {
    select! {
        _ = TaskOne() => println!("Task One Finished First"),
        _ = TaskTwo() => println!("Task Two Finished First"),
    }
}
