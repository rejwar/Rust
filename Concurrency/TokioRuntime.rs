use tokio::task;

async fn ComputeAsync() -> i32 {
    42
}

#[tokio::main]
async fn main() {
    let Result = task::spawn(ComputeAsync()).await.unwrap();
    println!("Computed Value: {}", Result);
}
