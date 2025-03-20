fn main() {
    let adder = make_adder(5);
    println!("Result: {}", adder(10));
}

fn make_adder(x: i32) -> impl Fn(i32) -> i32 {
    move |y| x + y
}
