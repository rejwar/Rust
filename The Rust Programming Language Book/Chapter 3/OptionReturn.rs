fn safe_divide(a:i32 , b:i32) -> Option<i32> {
    if b == 0 {
        None
    } else {
        Some(a / b)
    }
}

fn main() {
    match safe_divide(10, 2){
        Some(result) => println!("Result is {}",result),
        None => println!("Invalid division"),
}
}
