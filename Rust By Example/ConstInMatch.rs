const ERROR_CODE: u32 =404;

fn handle(code: u32 ) {
    match code {
        ERROR_CODE => println!("Not Found"),
        _=> println!("Other"),
    }
}
