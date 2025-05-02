enum Traffic Light {
    Red,
    Green,
    Yellow,
}

fn ShowSignal(signal: TrafficLight) {
    match signal {
        TrafficLight::Red  => println!("Stop"),
        TrafficLight::Green => println!("Go"),
        TrafficLight::Yellow => println!("Wait"), 
    }
}

fn main() {
    let Light: TrafficLight = TrafficLight::Red;
    ShowSignal(Light);
}
