macro_rules! ExprWithMessage {
    ($Expression:expr) => {
        println!("Result: {} = {}", stringify!($Expression), $Expression);
    };
}

fn main() {
    ExprWithMessage!(2 + 2);
    ExprWithMessage!(10 / 2);
}
