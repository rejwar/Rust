fn main() {
    let result = default_value();
    println!("Default value: {}", result);
}

fn default_value() -> i32 {
    42 // No semicolon means this is the returned value
}
