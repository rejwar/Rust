macro_rules! ExpressionSummer {
    ($X:expr ) => {$X};
    ($X: expr , $($Rest:expr), +) => {
        $X + ExpressionSummer! ($($Rest ) , +)
    };
}

fn main() {
    let Total = ExpressionSummer!(1,2,3,4,6,7,8);
    println!(" {}", Total);
}