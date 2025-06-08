use std::io;

fn main() {
    let Secret = 3;

    for _ in 0..5 {
        println!("Enter your guess:");

        let mut Guess = String::new();
        io::stdin().read_line(&mut Guess).expect("Failed to read input");

        let GuessNum: i32 = Guess.trim().parse().expect("Invalid number");

        if GuessNum == Secret {
            println!("🎉 Correct Guess! Exiting...");
            return;
        } else {
            println!("❌ Incorrect, try again!");
        }
    }
    println!("💀 Out of attempts! The correct number was {}.", Secret);
}
