macro_rules! SumAll {
    ( $( $x:expr ),* ) => {
        {
            let mut total = 0;
            $(
                total += $x;
            )*
            println!("The sum is: {}", total);
        }
    };
}

fn main() {
    SumAll!(3, 4, 5);         // Output: The sum is: 12
    SumAll!(10, 20, 30, 40);  // Output: The sum is: 100
}
