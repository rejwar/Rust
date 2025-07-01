#[derive(Debug)]

struct  S {
    x: i32 ,
}

fn main() {
    let value = 42;
    let s = S {x: value};
    println!("{:?}", s);
}
