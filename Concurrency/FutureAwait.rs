use std::future::Future;

fn CreateFuture() -> impl Future<Output = i32> {
    async { 100 }
}

#[tokio::main]
async fn main() {
    let Result = CreateFuture().await;
    println!("Future Result: {}", Result);
}
