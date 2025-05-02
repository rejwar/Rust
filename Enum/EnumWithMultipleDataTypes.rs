enum Status {
    Success(String),
    Error(u32 , String),
}

fn main() {
    let Response: Status = Status::Error(404, String::from("Not Found"));

    match Response {
        Status::Success(Message) => println!("Success : {}" , Message),
        Status::Error(Code , Description  ) => println!("Error :{} {} " ,Code , Description),
    }
}
