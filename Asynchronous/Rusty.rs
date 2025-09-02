use tokio::sync::mpsc;

#[tokio::main]

async fn main() {
    let (tx , mut rx) = mpsc::channel(2);

    tx.send("hi").await.unwrap();
    tx.send("hello").await.unwrap();

    println!("Bufffer full now");
    println!("Got {}", rx.recv().await.unwrap());
}use tokio::sync::mpsc;

#[tokio::main]

async fn main() {
    let (tx , mut rx) = mpsc::channel(2);

    tx.send("hi").await.unwrap();
    tx.send("hello").await.unwrap();

    println!("Bufffer full now");
    println!("Got {}", rx.recv().await.unwrap());
}