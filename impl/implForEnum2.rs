enum Status {
    Success,
    Error(String),
}
impl Status {
    fn Message (&self) -> Status {
        match self {
            Status::Success => "Success".to_string(),
            Status::Error(msf) => format!("Error {}", msg),
        }
    }
}