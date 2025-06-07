use std::future::Future;
use std::pin::Pin;

async fn SayHello() -> String {
    "Hello, World!".to_string()
}


fn main() {
    let future: Pin<Box<dyn Future<Output = String>>> = Box::pin(SayHello());
    let runtime =tokio::runtime::Runtime::new().unwrap();
    let Result = runtime.block_on(future);
    println!("{}", Result);
}
