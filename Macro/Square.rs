macro_rules! PrintSquare {
    ($Number:expr) => {
        println!("The square is: {}", $Number * $Number);
    };
}

fn main() {
    PrintSquare!(5);
    PrintSquare!(12);
}
