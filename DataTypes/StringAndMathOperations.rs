fn main() {
    let value:i32 = -15;
    println!("The absolute value of {} is {}", value, value.abs());
    
    let empty_space: &str = " my content  ";
    println!("Trimmed string: '{}'", empty_space.trim());

    println!("{}", value.pow(2))
}
