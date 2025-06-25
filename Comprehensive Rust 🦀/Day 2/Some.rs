use std::vec::Splice;

fn main() {
    let Number : Option<i32>  = Some(42);

    match Number {
        Some(Value) => println!("Value is {}",Value),
        None => println!("No value")
    }
}
