fn main() {
    let adder = make_adder(10);
    println!("10 + 5 = {}", adder(5));
}

fn make_adder(x: i32) -> impl Fn(i32) -> i32 {
    move |y| x + y
}
