fn IfLetOk() {
    let result: Result<u32 , &str> = Ok(100);

    if let Ok(val) = result {
        println!("Success {}", val);
    } else {
        println!("There was an error");
    }
}

fn main() {
    IfLetOk();
}
