fn identity<T>(value: T) -> T {
    value
}

fn main() {
    println!("{}", identity(5));
    println!("{}", identity("Rust"));
}
