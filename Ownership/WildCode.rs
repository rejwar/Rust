enum TrafficLight {
    Red,
    Yellow,
    Green,
}

fn print_code(c: TrafficLight) {
    match c {
        TrafficLight::Red => println!("stop"),
        TrafficLight::Yellow => println!("wait"),
        TrafficLight::Green => println!("go"),
    }
}
