enum TrafficLight {
    Red(u32),
    Green(u32),
    Yellow(u32),
}

impl TrafficLight {
    fn Duration(&self) -> u32{
        match self {
            TrafficLight::Red(Time) => *Time,
            TrafficLight::Green(Time) =>*Time,
            TrafficLight::Yellow(Time ) =>*Time,
        }
    }
}


fn main() {
    let Light: TrafficLight = TrafficLight::Red(30);
    println!("Red light duration {} seconds " , Light.Duration());
}
