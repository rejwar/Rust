fn print_twice< T: std::fmt::Debug> (value : T) {
    println!("{:?} , {:?}", value, value);
}

fn main() {
    print_twice(42);
    print_twice("Rustacean");
}
