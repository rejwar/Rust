fn main() {
    let integer = 10;
    let float = integer as f32;
    println!("Integer : {}  , Float : {}", integer, float);

    let another_float = 3.1416;
    let truncated_integer  = another_float as i32 ;
    println!("Float  {} , Integer {}", another_float, truncated_integer);

    let large_integer = 1000;
    let smaller_integer= large_integer as u8;

    println!("Large integer {}, smaller integer {}", large_integer , smaller_integer);
}