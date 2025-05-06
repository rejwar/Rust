trait Displayable {
    fn Show(&self);
}

impl<T: std::fmt::Debug> Displayable for T {
    fn Show(&self) {
        println!("{:?}", self);
    }
}

fn main() {
    42.Show();
    "Rust".Show();
}
