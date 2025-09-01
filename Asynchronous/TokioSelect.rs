use tokio::time::{sleep, Duration};

#[tokio::main]

async fn main() {
    tokio::select! {
        _= sleep(Duration::from_secs(1)) => println!("timer 1 finished first"),
        _= sleep(Duration::from_secs(2)) => println!("Timer 2 finished later "),
    }
}