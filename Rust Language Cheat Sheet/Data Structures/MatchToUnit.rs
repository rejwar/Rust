fn HandleCommand(cmd: &str) {
    match cmd {
        "start" => println!("Starting"),
        "stop" => println!("Stopping"),
        _=>(),
    }
}

fn main() {
    HandleCommand("Foo");
}
