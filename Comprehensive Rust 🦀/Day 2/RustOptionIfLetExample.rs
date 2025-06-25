 fn main() {
    let Number: Option<i32> = Some(40);

    if let Some(Value) = Number  {
        println!("Value is {}", Value);
    } else {
        println!("No Value");
    }
}
