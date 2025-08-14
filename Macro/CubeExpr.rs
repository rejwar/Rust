macro_rules! CubeExpr {
    ($Number:expr) => {
        println!("{} = {}", stringify!($Number), $Number * $Number * $Number);
    };
}

fn main() {
    CubeExpr!(3);
    CubeExpr!(2 + 2);
}