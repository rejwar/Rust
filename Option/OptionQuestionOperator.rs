fn get_optional_value(flag: bool) -> Option<i32> {
    if flag {
        Some(42)
    } else {
        None
    }
}

fn process_value(value: Option<i32>) {
    match value {
        Some(v) => println!("Value is: {}", v),
        None => println!("No value provided"),
    }
}

fn main() {
    println!("Starting the program...");
}
