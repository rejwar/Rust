async fn add (x: i32 , y: i32 ) -> i32 {
    x + y
}

#[tokio::main]

async fn main() {
    let sum = add(5, 7).await;
    println!("Sum = {}", sum);

}