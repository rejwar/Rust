enum VehicleType{
    Car,
    Bike,
}

struct Vehicle{
    name: String,
    speed: u32 ,
    vehicle_type: VehicleType,
}

trait Describable {
    fn describe(&self);
}

impl Describable for Vehicle {
    fn describe (&self) {
        match self.vehicle_type {
            VehicleType::Car =>{
                println!("{} is car with speed {} km/h " , self.name , self.speed);
            }
            VehicleType::Bike => {
                println!("{} is a bike with speed {} km/h" , self.name , self.speed);
            }
        }
    }
}

impl Vehicle {
    fn increse_speed(&mut self , value: u32) {
        self.speed += value;
    }
}

fn main() {
    let mut car = Vehicle {
        name: String::from("Toyota"),
        speed: 80,
        vehicle_type:VehicleType::Car,
    };

    let bike = Vehicle {
        name: String::from("Yamaha"),
        speed: 60,
        vehicle_type: VehicleType::Bike,
    };

    car.describe();
    bike.describe();

    car.increse_speed(20);
    car.describe();
}