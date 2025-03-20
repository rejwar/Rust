struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let mut rect = Rectangle { width: 10, height: 20 };
    let reference = &mut rect;
    reference.width = 15;
    println!("Updated Rectangle: {} x {}", reference.width, reference.height);
}
