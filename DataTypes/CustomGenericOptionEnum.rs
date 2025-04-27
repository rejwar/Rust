use std::fmt;

#[derive(Debug)]
enum MyOption<T> {
    Some(T),
    None,
}

fn main() {
    let some_number = MyOption::Some(5);
    let some_string = MyOption::Some("Hello");

    println!("Number: {:?}", some_number);
    println!("String: {:?}", some_string);
}
