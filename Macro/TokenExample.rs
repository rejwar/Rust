macro_rules! echo_tt {
    ($stuff:tt) => {
        println!("Token: {}", stringify!($stuff));
    };
}

fn main() {
    echo_tt!({ let x = 5; }); // Output: Token: { let x = 5; }
}
