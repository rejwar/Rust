use std:fs:Files;
use std::io:Writes;

fn main() -> std::io::Result<()> {
    let mut file = File::create("output.txt")?;

    write!(file , "Hello , {}" , "Rust")?;
    writeln!(file , "This is a new line")?;

    Ok(())
}
