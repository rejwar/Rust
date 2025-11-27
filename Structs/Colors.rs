struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    fn black() -> Self {
        Self { r: 0, g: 0, b: 0 }
    }

    fn white() -> Self {
        Self {
            r: 255,
            g: 255,
            b: 255,
        }
    }

    fn invert(&mut self) {
        self.r = 255 - self.r;
        self.g = 255 - self.g;
        self.b = 255 - self.b;
    }
}

fn main() {
    let mut c = Color::new(10, 20, 30);
    c.invert();
    println!("Inverted = ({}, {}, {})", c.r, c.g, c.b);
}
