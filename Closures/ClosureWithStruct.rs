struct Calculator<F: Fn(i32, i32) -> i32> {
    Operation: F,
}

fn main() {
    let Add = Calculator { Operation: |a, b| a + b };
    println!("Addition Result: {}", (Add.Operation)(5, 10));
}
