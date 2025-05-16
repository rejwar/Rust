use std::fmt::Write;

fn main() {
    let mut output = String::new();

    write!(output , "Binary : {:b} , Hex {:x}" , 10 , 255).unwrap();
    println!("{}" , output);
}
