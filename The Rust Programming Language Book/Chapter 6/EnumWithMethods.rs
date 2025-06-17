// 

enum Status {
    Success,
    Error(String),
}

impl Status {
    fn is_ok(&self) -> bool {
        matches!(self, Status::Success)
    }
}

fn main() {
    let result = Status::Error(String::from("Oops"));
    println!("Is success? {}", result.is_ok());
}
