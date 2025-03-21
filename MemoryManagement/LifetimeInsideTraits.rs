trait Logger<'a> {
    fn log(&self, message: &'a str);
}

struct Console;

impl<'a> Logger<'a> for Console {
    fn log(&self, message: &'a str) {
        println!("Log: {}", message);
    }
}

fn main() {
    let console = Console;
    console.log("Hello, Logger!");
}
