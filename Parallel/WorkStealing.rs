use tokio::task;

#[tokio::main]
async fn main() {
    let Task1 = task::spawn_blocking(|| {
        println!("Heavy CPU computation running in a separate worker thread");
    });

    let Task2 = task::spawn(async {
        println!("Light async computation running");
    });

    Task1.await.unwrap();
    Task2.await.unwrap();
}
