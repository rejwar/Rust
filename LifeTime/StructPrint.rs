use std::fmt::{Display, Debug};

fn show<T: Display + Debug>(item: T) {
    println!("Display: {}", item);
    println!("Debug: {:?}", item);
}

fn main() {
    let value = 42;
    show(value);
    
    let text = "Hello, world!";
    show(text);
}