use std::{io, sync::mpsc::SendError};

fn main() {
    let SecretNumber:i32 = 5;
    let mut GuessNumber:i32 = 0;

    while GuessNumber != SecretNumber {
        println!("Enter your guess :");

        let mut Input = String::new();
        io::stdin().read_line(&mut Input).expect("Failed to read Input");
        GuessNumber = Input.trim().parse().expect("Invalid Number");

        if GuessNumber != SecretNumber  {
            println!("Incorret ! Try again ");
        }

    }
    println!("Correct Guess");
}
