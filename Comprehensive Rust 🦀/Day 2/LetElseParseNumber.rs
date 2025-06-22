
use std::str::FromStr;

fn parse_u32(input: &str) -> u32 {
    let Ok(n) = u32::from_str(input) else {
        panic!("Failed to parse '{}' as u32", input);
    };
    n
}

fn main() {
    let number = parse_u32("42");
    println!("Parsed number: {}", number);
}
