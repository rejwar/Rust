struct TeslaCar {
    model: String,
    battery_level: i32,
}

fn main() {
    let my_car = TeslaCar {
        model: String::from("Model S"),
        battery_level: 100,
    };

    let your_car = my_car;
    println!("{}", your_car.model);
}
