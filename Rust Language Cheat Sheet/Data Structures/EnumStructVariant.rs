enum Response {
    Success {code : u16 , messages : String },
    Error {code: u16 , reason: String},
}

fn main() {
    let res1 = Response::Success { code: 200, messages: String::from("Okk"), };

    let res2 = Response::Error { code: 500, reason: String::from("Server Error")};

    match res1

     {
        Response::Success { code, messages } => {
                println!("Suceess code {} {}", code , messages);
        }
        Response::Error { code, reason } => {
            println!("Error {} {}", code , reason);
        }
    }
}
