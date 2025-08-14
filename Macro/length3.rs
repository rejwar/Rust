macro_rules! PrintTextLength {
    ($Text:expr) => {
        println!("The length is: {}", /* ??? */ );
    };
}

fn main() {
    PrintTextLength!("Rifat");
    PrintTextLength!("Hello, World!");
}
