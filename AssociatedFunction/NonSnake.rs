#![allow(non_snake_case)]
struct Config {
    MaxUser: u32,
}

impl Config {
    fn DefaultUser() -> u32 {
        100 // constant মান ফেরত দিচ্ছে
    }
}
fn main() {
    println!("Default max users: {}", Config::DefaultUser());
}
