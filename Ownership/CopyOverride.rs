#[derive(Debug, Clone, Copy)]

struct Tiny {
    val: i32,
}

fn main() {
    let t1 = Tiny { val: 1 };
    let t2 = t1;
    println!("t1 is still here {}", t1);
}
