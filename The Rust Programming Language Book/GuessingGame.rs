use std::io;

fn main() {
    println!("Guess The Number!");
    println!("Please enter your guess:");

    let mut guess:String = String::new();

    io::stdin().read_line(&mut guess).expect("Failed to read line");

    println!("You guessed: {}", guess.trim());

}
