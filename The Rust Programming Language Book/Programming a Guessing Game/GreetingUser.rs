use std::io;

fn main() {
    println!("Enter your Number:");

    let mut Name:String = String::new();
    io::stdin().read_line(&mut Name).expect("Failed to read line");

    println!("You entered: {}", Name.trim());

}
