fn main() {
    let Multiply = |x: i32 , y: i32 | {
        let result = x * y;
        println!("result {}", result);
        result
    };

    Multiply(4,5);
}