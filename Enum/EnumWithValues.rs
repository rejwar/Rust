enum Color {
    Red(u8, u8, u8),
    Green,
    Blue,
}

fn main() {
    let color = Color::Red(255, 0, 0);
    match color {
        Color::Red(r, g, b) => println!("Red: {}, Green: {}, Blue: {}", r, g, b),
        Color::Green => println!("Green color"),
        Color::Blue => println!("Blue color"),
    }
}
