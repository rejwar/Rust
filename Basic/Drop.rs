struct SBox<T>(T);

impl<T> SBox<T> {
    fn new(x: T) -> Self {
        Self(x)
    }
}

fn main() {
    let x = 10;
    let y = SBox::new(10);

    println!(" {x}");
}
