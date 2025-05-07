#[macro_export]
macro_rules! WelcomeMessage {
    () => {
        println!("Welcome to Advanced Rust!");
    };
}

fn main() {
    WelcomeMessage!();
}
