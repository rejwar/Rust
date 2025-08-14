practice  1 
macro_rules! DoubleExpr {
    ($Number:expr) => {
        println!("{} = {}", stringify!($Number), $Number *2 );
    };
}

fn main() {
   DoubleExpr!(5);
    DoubleExpr!(2 + 3);
}