trait Printable {
    fn Print(&self);
}

impl Printable for i32 {
    fn Print(&self) {
        println!(" Integer : {}", self);
    }
}

impl Printable for String {
    fn Print (&self) {
        println!("String {}", self);
    }
}

fn main() {
    let Number = 42;
    let Text = String::from("Hello");

    Number.Print();
    Text.Print();
}
