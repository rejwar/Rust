fn pattern_match_result() {
    let result: Result<i32, &str> = Ok(5);

    // if let ব্যবহার করে
    if let Ok(value) = result {
        println!("Value is {}", value);
    }

    let res: Result<i32, &str> = Err("Oops");

    // match ব্যবহার করে
    match res {
        Ok(v) => println!("Got {}", v),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    pattern_match_result();
}
