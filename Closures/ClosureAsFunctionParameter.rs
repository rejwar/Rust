fn ExecuteClosure<F: Fn(i32)>(Closure: F) {
    Closure(10);
}

fn main() {
    let PrintNumber = |x: i32| println!("Number: {}", x);
    ExecuteClosure(PrintNumber);
}
