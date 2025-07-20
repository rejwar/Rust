enum Status  {
    Success,
    Error(String),
}

impl Status  {
    fn Messages (&self) -> String {
        match self {
            Status::Success => "Operation Succeeded ". to_string(),
            Status::Error(msg) => format!("Error {}", msg),
        }
    }
}

fn main() {
    let s1 = Status::Success;
    let s2 = Status::Error("File not found ".to_string());

    println!("{}", s1.Messages());
    println!("{}", s2.Messages());
}