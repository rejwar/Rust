use std:: ops::Deref;

struct  Wrapper(String);

impl Deref for Wrapper {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn greet(name: &str) {
    println!("Hello {}", name);
}

fn main() {
    let w = Wrapper(String::from("Md"));
    greet(&w);
}
