fn main() {
    let b = creat_box();
    println!("Heap Value {}", b);
}

fn creat_box() -> Box<i32> {
    Box::new(99)
}
