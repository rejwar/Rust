macro_rules! SquareExpr {
    ($Number:expr) => {
        println!("{} = {}", stringify!($Number), $Number * $Number);
    };
}

fn main() {
    SquareExpr!(3);
    SquareExpr!(2 + 2);
}
