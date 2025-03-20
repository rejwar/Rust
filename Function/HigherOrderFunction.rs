fn main() {
    let add_one = |x: i32| x + 1;
    let result = apply(10, add_one);
    println!("Result: {}", result);
}

fn apply(x: i32, f: fn(i32) -> i32) -> i32 {
    f(x)
}
