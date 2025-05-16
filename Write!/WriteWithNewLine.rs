use std::fmt::Write;

fn main() {
    let mut output = String::new();

    writeln!(output , "Hello , {}!" , "World" ).unwrap();
    writeln!(output , "This is Rust programming ") .unwrap();

    println!("{}" , output);
}
