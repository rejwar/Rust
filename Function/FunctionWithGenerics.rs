fn main() {
    let int_result = identity(42);
    let str_result = identity("Hello");
    println!("IntResult: {}, StrResult: {}", int_result, str_result);
}

fn identity<T>(value: T) -> T {
    value
}
