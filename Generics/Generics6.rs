use std::fmt::Debug;

fn print_debug<T: Debug> (value: T ){
    println!("{:?}", value);
}

fn main() {
    print_debug(10);
    print_debug("World");
    print_debug("Cij");
}