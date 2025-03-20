fn main() {
    let result = square(5);
    println!("Square: {}", result);
}

fn square(num: i32) -> i32 {
    num * num
}
