use std::fmt::Write;

fn main() {
    let mut output = String::new();

    write!(output , "Hello {}!" , "World").unwrap();
    println!("{}" , output);
}
