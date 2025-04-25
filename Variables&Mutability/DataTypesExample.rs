fn main() {
    let eight_bit: i8   = -112;
    let sixteen_bit: i16 = -112;
    let thirty_two_bit: i32 = -112;
    let sixty_four_bit: i64 = -112;
    let thirty_two_bit_unsigned: u32 = 112;
    let sixty_four_bit_unsigned: u64 = 112;
    let thirty_two_bit_float: f32 = -112.0;
    let sixty_four_bit_float: f64 = -112.0;
    let boolean: bool = true;
    let character: char = 'a';
    let string: &str = "Hello, world!";
    let array: [i32; 5] = [1, 2, 3, 4, 5];
    let tuple: (i32, f64, char) = (1, 2.0, 'a');

    println!("8-bit signed integer: {}", eight_bit);
    println!("16-bit signed integer: {}", sixteen_bit);
    println!("32-bit signed integer: {}", thirty_two_bit);
    println!("64-bit signed integer: {}", sixty_four_bit);
    println!("32-bit unsigned integer: {}", thirty_two_bit_unsigned);
    println!("64-bit unsigned integer: {}", sixty_four_bit_unsigned);
    println!("32-bit float: {}", thirty_two_bit_float);
    println!("64-bit float: {}", sixty_four_bit_float);
    println!("Boolean: {}", boolean);
    println!("Character: {}", character);
    println!("String: {}", string);
    println!("Array: {:?}", array);
    println!("Tuple: {:?}", tuple);
}
