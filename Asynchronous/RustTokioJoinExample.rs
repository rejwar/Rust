use tokio::time::{sleep , Duration};

async fn task (name: &str) {
    println!("name {} start ", name);
    sleep(Dcuration::from_secs(2)).await;
    println!("{} name end ", name);
}

#[tokio::main]
async fn main() {
    tokio::join!(task("A"), task("B"));
}