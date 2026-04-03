fn main() {
    let some_value = Some(100);

    if let Some(value) = some_value {
        println!(" Value {}", value);
    } else {
        println!(" No value ");
    }
}
