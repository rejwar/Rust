use std::path::StripPrefixError;

enum Status {
    Success(String),
    Error(u32 , String),
}

fn CheckStatus( response: Status) {
    match response {
        Status:: Success(message) => println!("Success {}" , message),
        Status::Error(code , description) => println!("Error {} {}" , code, description),
    }
}

fn main() {
    let Response: Status = Status::Error(404, String::from("Not found"));
    CheckStatus(Response);
}
