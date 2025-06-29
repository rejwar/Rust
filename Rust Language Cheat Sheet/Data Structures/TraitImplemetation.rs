struct  Logger {} 

impl Logger {
    fn log(&self, message: &str ) {
        println!(" LOg : {}", message);
    }
}

fn main() {
    let logger = Logger{} ;
    logger.log("This is a message");
}
