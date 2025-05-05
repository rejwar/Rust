fn GenerateMultiplier() -> impl Fn(i32) -> i32 {
    let Factor = 2;
    move |x| x * Factor
}

fn main() {
    let MultiplyByTwo = GenerateMultiplier();
    println!("Result: {}", MultiplyByTwo(5));
}
