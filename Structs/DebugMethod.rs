struct Car {
    make: String,
    year: u32,
}

impl Car {
    fn debug(&self) {
        println!("Car: Make: {}, Year: {}", self.make, self.year);
    }
}

fn main() {
    let car = Car {
        make: String::from("Toyota"),
        year: 2020,
    };
    car.debug();
}
