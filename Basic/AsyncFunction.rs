use tokio::time::{sleep, Duration};

async fn async_task() {  // ✅ ফাংশনকে async করতে হবে
    println!("Starting async task..");
    sleep(Duration::from_secs(2)).await;  // ✅ await সঠিকভাবে কাজ করবে
    println!("Async task completed");
}

#[tokio::main]
async fn main() {
    async_task().await;  // ✅ ফাংশনটি await করা হয়েছে
}
