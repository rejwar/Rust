fn apply_twice(f: fn(i32) -> i32, x: i32) -> i32 {
    f(f(x))
}

fn square(x: i32) -> i32 {
    x * x
}

fn main() {
    let result = apply_twice(square, 3);
    println!("Result: {}", result);
}
