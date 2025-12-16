trait Printer {
    fn print(&self);
}

struct Message<'a> {
    text: &'a str,
}

impl<'a> Printer for Message<'a> {
    fn print(&self) {
        println!(" Message is {}", self.text);
    }
}

fn main() {
    let s = String::from("Hello rejwar");
    let msg = Message { text: &s };

    let p: &dyn Printer = &msg;
    p.print();
}
