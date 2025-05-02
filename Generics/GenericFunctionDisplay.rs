use std::fmt::Debug;

fn print_value<T: Debug>(value: T) {
    println!("{:?}", value);
}

fn demonstrate_generics() {
    print_value(42);          // i32
    print_value("Hello");     // &str
    print_value(3.1);         // f64
    print_value(vec![1, 2, 3]); // Vec<i32>
}

fn main() {
    demonstrate_generics();
}
