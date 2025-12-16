struct Containers<'a> {
    data: &'a str,
}

impl<'a> Containers<'a> {
    fn combine<'b>(&self, other: &'b str) -> String {
        format!(" {} + {} ", self.data, other)
    }
}

fn main() {
    let a = String::from("Rust");
    let b = String::from("Lifetime");
    let c = Containers { data: &a };

    let result = c.combine(&b);
    println!("{}", result);
}
