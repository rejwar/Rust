macro_rules! GreetTwoNames {
    ($Name1:expr, $Name2:expr) => {
        println!("Hello, {}!", $Name1);
        println!("Hello, {}!", $Name2);
    };
}

fn main() {
    GreetTwoNames!("Rifat", "Rahman");
}