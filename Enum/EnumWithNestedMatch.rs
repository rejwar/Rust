enum Vehicle {
    Car { make: String },
    Bike,
}

fn main() {
    let vehicle = Vehicle::Car { make: String::from("Toyota") };
    match vehicle {
        Vehicle::Car { make } => println!("Car Make: {}", make),
        Vehicle::Bike => println!("It's a bike!"),
    }
}
