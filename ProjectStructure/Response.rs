pub enum Status {
    Success(String),
    Error(u32, String),
}

pub fn GetResponse() -> Status {
    Status::Success(String::from("Operation completed!"))
}
