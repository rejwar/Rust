#[derive(Debug)]

struct S {
    x: i32,
}

fn main() {
    let x = 99;
    let s = S {x};
    println!("{:?}", s);
}
