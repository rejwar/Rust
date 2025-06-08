use std::io;

fn main() {
    loop {
        println!("Enter a valid number:");

        let mut Input = String::new();
        io::stdin().read_line(&mut Input).expect("Failed to read input");

        match Input.trim().parse::<i32>() {
            Ok(Number) => {
                println!("Valid input received: {}", Number);
                break; // Exit loop when valid
            }
            Err(_) => println!("❌ Invalid number, please try again."),
        }
    }
}
