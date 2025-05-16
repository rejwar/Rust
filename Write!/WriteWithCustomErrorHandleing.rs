use std::fmt::Write;

fn main() {
    let mut buffer = String::new();

    if let Err(e) = write!(buffer , "Error code: {}" , 404) {
        eprintln!("Failed to write to buffer : {}" , e);
    } else {
        println!("Buffer content {}" , buffer);
    }
}
