use std::io::{self, Write};

fn main()-> io::Result<()> {
    let mut input = String::new();
    println!("Enter a string:");
    io::stdin().read_line(&mut input)?;
    
    let trimmed_input = input.trim();
    if trimmed_input.is_empty() {
        println!("You entered an empty string.");
    } else {
        println!("You entered: {}", trimmed_input);
    }
    
    Ok(())

}
