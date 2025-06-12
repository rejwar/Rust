use std::io;

fn main() {
    let SecretNumber = 42;
    let mut GuessNumber = 0;

    while GuessNumber != SecretNumber {
        println!("Enter your guess:");

        let mut Input = String::new();
        io::stdin().read_line(&mut Input).expect("Failed to read input");

        GuessNumber = Input.trim().parse().expect("Invalid number");

        if GuessNumber != SecretNumber {
            println!("Incorrect! Try again.\n");
        }
    }

    println!("✅ Correct guess!");
}
