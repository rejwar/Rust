enum Status {
    Success,
    Error(String),
}

impl Status {
    fn report(&self) {
        match self {
            Status::Success => println!("All good!"),
            Status::Error(msg) => println!("Error: {}", msg),
        }
    }
}

fn main() {
    let result = Status::Error(String::from("File not found"));
    result.report();
}
