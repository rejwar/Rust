enum Device {
    Phone(String),
    Laptop,
}

fn main() {
    let device = Device::Phone(String::from("iPhone"));
    match device {
        Device::Phone(model) => println!("Phone: {}", model),
        _ => println!("Unknown device"),
    }
}
