macro_rules! ShowExpr {
    ($Expression:expr) => {
        println!("{:?} = {:?}", stringify!($Expression), $Expression);
    };
}

fn main() {
    ShowExpr!(2 + 3);
    ShowExpr!(10 / 2);
}
