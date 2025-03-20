struct Car {
    Make: String,
    Model: String,
    Year: u32,
}

fn main() {
    let MyCar = Car {
        Make: String::from("Toyota"),
        Model: String::from("Corolla"),
        Year: 2020,
    };
    println!("Car: {} {} {}", MyCar.Make, MyCar.Model, MyCar.Year);
}
