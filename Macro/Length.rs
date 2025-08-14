macro_rules! PrintTextLenght {
    ($text:expr) => {
        println!("The lenght is {}", $text.len())
    };
}

fn main() {
    PrintTextLenght!("Newb");
    PrintTextLenght!("Hello");
}