mod response;

fn main() {
    let Result = response::GetResponse();
    match Result {
        response::Status::Success(msg) => println!("Success: {}", msg),
        response::Status::Error(code, description) => println!("Error {}: {}", code, description),
    }
}
