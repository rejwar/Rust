#[derive(Debug)]

struct S<T> {
    x: T,
}

fn main() {
    let int_struct = S { x : 32};
    let str_struct = S { x : "Rust"};


    println!(" {:?}", int_struct);
    println!( "{:?}", str_struct);
}
