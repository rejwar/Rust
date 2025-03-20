fn main() {
    let mut number = 5;
    increment(&mut number);
    println!("Number: {}", number);
}

fn increment(value: &mut i32) {
    *value += 1;
}
