fn main() {
    let result = factorial(5);
    println!("Factorial: {}", result);
}

fn factorial(n: u32) -> u32 {
    if n == 0 {
        1
    } else {
        n * factorial(n - 1)
    }
}
