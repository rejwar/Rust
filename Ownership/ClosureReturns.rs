fn make_adder(x: i32) -> impl Fn(i32) -> i32 {
    move |n| n + x
}

fn main() {
    let add10 = make_adder(10);
    println!("{}", add10(5)); // 15
}
