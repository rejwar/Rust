fn main() {
    let value = 42;
    print_value(&value);
}

fn print_value(input: &i32) {
    println!("Value: {}", input);
}
