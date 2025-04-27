use std::fmt::Debug;

fn display<T: Debug>(item: T) {
    println!("item: {:?}", item);
}

fn main() {
    display(42);
    display("Hello");
    display(3.14);
}
