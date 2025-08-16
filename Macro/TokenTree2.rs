macro_rules! TokenCounter {
    () => {0};
    ($_Head:tt $($Tail:tt)*) =>  {
            1 + TokenCounter!($($Tail)*)
    };
}

fn main() {
    let Count = TokenCounter!(A B C D E F );
    println!("{}", Count);
}