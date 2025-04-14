use std::fmt::format;

enum Status{
    Success,
    Failure(String),
}

fn CheckStatus (code: i32 ) ->Status {
    if code == 200{
        Status::Success

    }else {
        Status::Failure(format!("Code: {}" , code ))
    }
}

fn main() {
    let Result = CheckStatus(500);

    match Result {
        Status::Success => println!("Ok"),
        Status::Failure(msg) => println!("error: {}",msg),
    }

}
