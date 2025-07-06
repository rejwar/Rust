#[derive(Debug)]

enum Color {
    Red,
    Green,
    Blue,
}

fn main() {
    let color = Color::Green;
    let color_value = color as i32;
    println!("Color : {:?}, value {}", color, color_value);
}
