struct Color(u8, u8, u8);

fn PrintColor(c: Color) {
    println!("Red: {}, Green: {}, Blue: {}", c.0, c.1, c.2);
}

fn main() {
    let red = Color(255, 0, 0);
    PrintColor(red);
}
