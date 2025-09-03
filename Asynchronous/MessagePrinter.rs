use std::fmt::format;

use tokio::sync::mpsc;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let (tx , mut rx ) = mpsc::channel(10);

    let tx1 = tx.clone();
    tokio::spawn(async move {
        for i in 1..=3 {
            tx1.send(format!("Messages {} from A ", i)).await.unwrap();
            sleep(Duration::from_millis(500)).await;

        }
    });

tokio::spawn(async move {
    for i in 1..= 3 {
        tx.send(format!("Message {} form B" , i)).await.unwrap();
        sleep(Duration::from_millis(500)).await;
    }
});


while let   Some(msg)=rx.recv().await  {
    println!("GOt : {}", msg);
}
}