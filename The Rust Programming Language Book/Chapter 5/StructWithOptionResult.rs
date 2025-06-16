// How to use struct with Option and Result?

struct Config {
    username: Option<String>,
    retry_count: Result<u8, String>,
}

fn UseOptionResultInStruct() {
    let config = Config {
        username: Some(String::from("admin")),
        retry_count: Ok(3),
    };

    if let Some(name) = config.username {
        println!("Username: {}", name);
    }

    match config.retry_count {
        Ok(count) => println!("Retries: {}", count),
        Err(err) => println!("Error: {}", err),
    }
}
