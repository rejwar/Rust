use std::io::{self, Read};

fn main() {
    let Secret = 7;

    loop {
        println!("Enter your guess :");

        let mut Guess = String::new();
        io::stdin().read_line(&mut Guess). expect("Failed to read line");

        let GuessNum:i32 = Guess.trim().parse().expect("Invalid input, please enter a number");

        if GuessNum == Secret {
            println!("Congratulations! You guessed the secret");
            break;
    }else{
        println!("Wrong guess, try again!");
    }
}
}
