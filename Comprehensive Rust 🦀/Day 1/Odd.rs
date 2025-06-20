fn main() {
    let number = 15;

    let description = if number % 2 == 0 {
        "even"
    } else if number % 3 == 0 {
        "Odd and divisible by 3"
    } else {
        "Odd"
    };
    println!(   "{} and {}" , number, description);
}
