trait Vehicle {
    fn drive(&self);
}

struct Car;
struct Bike;

impl Vehicle for Car {
    fn drive(&self) {
        println!("Car is driving ");
    }
}

impl Vehicle for Bike {
    fn drive(&self) {
        println!("Bike is driving");
    }
}

fn get_vehicle(choice: u8) -> Box<dyn Vehicle> {
    match choice {
        1 => Box::new(Car),
        _ => Box::new(Bike),
    }
}

fn main() {
    let v1 = get_vehicle(1);
    let v2 = get_vehicle(2);

    v1.drive();
    v2.drive();
}
