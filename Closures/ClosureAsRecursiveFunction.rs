fn main() {
    let Factorial = |n: u32| -> u32 {
        (1..=n).product()
    };

    println!("Factorial: {}", Factorial(5));
}
