fn PrintValue<T>(value: T) {
    println!("Value : {}", value);
}

fn main() {
    PrintValue(5);
    PrintValue("Hello");
    PrintValue(3.14);
}
