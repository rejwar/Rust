struct Start {}
struct Stop {} 

fn control_system(state: &str) {
    match state {
        "start" => println!("System started"),
        "ended" => println!("System stopped"),
        _ => println!("Cant get"),
    }
}

fn main() {
    let start = Start {};
    let stop = Stop{};

    control_system("start");
    control_system("Stop");
}
