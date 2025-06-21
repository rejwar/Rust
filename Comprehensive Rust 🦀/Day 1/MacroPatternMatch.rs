macro_rules! Matcher {
    (add $a: expr , $b: expr) => {
        println!("The Result is {}", $a + $b);
    };

    (mul $a:expr , $b:expr ) => {
        println!("Result {}", $a * $b);
    };
}

fn main() {
    Matcher!(add 3,5);
    Matcher!(mul 6,7);
}
