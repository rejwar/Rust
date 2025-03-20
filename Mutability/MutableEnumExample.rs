enum Light {
    On,
    Off,
}

fn main() {
    let mut Status = Light::Off;
    Status = Light::On;
    println!("Light Status: {:?}", Status);
}
