use std::rc::Rc;

#[derive(Debug)]
enum TrafficState {
    Green,
    Red,
    Yellow,
}

impl TrafficState {
    fn next(&self) -> TrafficState {
        match self {
            TrafficState::Green => TrafficState::Yellow,
            TrafficState::Red =>  TrafficState::Green,
            TrafficState::Yellow => TrafficState::Red,
        }
    }
}

fn main() {
    let mut light = TrafficState::Green;

    for _ in 0..4 {
        println!("Current ! {:?}", light);
        light =light.next();
    }
}
