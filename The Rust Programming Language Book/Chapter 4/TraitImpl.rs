struct Logger;

trait Log {
    fn log(&self, msg: &str);
}

impl Log for Logger {
    fn log(&self, msg: &str) {
        println!("LOG: {}", msg);
    }
}

fn main() {
    let logger = Logger;
    logger.log("Hello from unit struct!");
}
