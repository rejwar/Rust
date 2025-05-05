fn Apply<T, F: Fn(T) -> T>(Value: T, Closure: F) -> T {
    Closure(Value)
}

fn main() {
    let Double = |x: i32| x * 2;
    println!("Doubled: {}", Apply(10, Double));
}
