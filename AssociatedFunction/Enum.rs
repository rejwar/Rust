enum Color {
    Red,
    Green,
    Blue,
}

impl Color {
    fn default() -> Color {
        // associated function
        Color::Red
    }
}

fn main() {
    let col = Color::default();
}
