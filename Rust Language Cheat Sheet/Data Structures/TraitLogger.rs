enum Logger {}

impl Logger {
    fn log(message: &str) {
        println!("Log: {}", message);
    }
}

fn main() {
    Logger::log("This is a log message!");
}
