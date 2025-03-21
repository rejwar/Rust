fn main() {
    let value: Option<i32> = Some(15);
    if let Some(v) = value {
        println!("Value: {}", v);
    } else {
        println!("No value found");
    }
}
