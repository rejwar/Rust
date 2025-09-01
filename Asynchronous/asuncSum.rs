#[tokio::main]

async fn main() {
    let handle = tokio::spawn(async {
        "Hello from spawned task!"
    });

    let msg = handle.await.unwrap();
    println!("{}", msg);
}x