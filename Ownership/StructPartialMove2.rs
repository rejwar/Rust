struct Car {
    name: String,
}

fn main() {
    let c = Car {
        name: String::from("Tesla"),
    };
    consume_struct(c);
}

fn consume_struct(car: Car) {
    println!("Driving {}", car.name);
}
