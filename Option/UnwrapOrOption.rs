fn main() {
    let value: Option<i32> = None;
    println!("Result is :{}", value.unwrap_or(99)) 
}
