use std::io;

fn main() {
    let Secret = 5;
    let mut GuessNum = 0;

    while GuessNum != Secret {
        println!("Enter your guess:");

        let mut Guess = String::new();
        io::stdin().read_line(&mut Guess).expect("Failed to read input");

        GuessNum = Guess.trim().parse().expect("Invalid number");

        if GuessNum == Secret {
            println!("🎉 Correct Guess!");
        } else {
            println!("❌ Try again!");
        }
    }
}
