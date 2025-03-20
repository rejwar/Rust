struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect = Rectangle {
        width: 10,
        height: 20,
    };
    let area = calculate_area(&rect);
    println!("Area: {}", area);
}

fn calculate_area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}
