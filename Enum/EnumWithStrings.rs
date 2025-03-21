enum Alert {
    Warning(String),
    Error(String),
}

fn main() {
    let alert = Alert::Error(String::from("Fatal Error"));
    match alert {
        Alert::Warning(message) => println!("Warning: {}", message),
        Alert::Error(message) => println!("Error: {}", message),
    }
}
