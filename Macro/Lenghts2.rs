macro_rules! PrintTextLengths {
    ( $( $Text:expr ),* $(,)? ) => {{
        $(
            println!("{:?} -> length: {}", $Text, $Text.len());
        )*
    }};
}

fn main() {
    PrintTextLengths!("Rifat", "Hi", "Rust Macro",);
}
