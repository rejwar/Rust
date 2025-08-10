#[derive(Debug)]
struct Counter {
    value : i32,
}

fn main() {
    let mut c = Counter  {value : 0};
    c.value += 1;
    println!("{:?}", c);
}