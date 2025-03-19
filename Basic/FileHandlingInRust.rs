use std::fs::File;
use std::io::{self, Write, Read};

fn main() -> io::Result<()> {
    // Writing to a file
    let mut file = File::create("example.txt")?;
    file.write_all(b"Hello, Rust!")?;

    // Reading from a file
    let mut file = File::open("example.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    println!("File contents: {}", contents);

    Ok(())
}
