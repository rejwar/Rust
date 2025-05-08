use tokio::task;

#[tokio::main]
async fn main() {
    let Task1 = task::spawn(async { println!("Task 1 Running"); });
    let Task2 = task::spawn(async { println!("Task 2 Running"); });

    Task1.await.unwrap();
    Task2.await.unwrap();
}
