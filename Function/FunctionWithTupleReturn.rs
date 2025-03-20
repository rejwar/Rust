fn main() {
    let (x, y) = swap(10, 20);
    println!("Swapped: x={}, y={}", x, y);
}

fn swap(a: i32, b: i32) -> (i32, i32) {
    (b, a)
}
