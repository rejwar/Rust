enum GIFT {
    Toy(String),
}

fn main() {
    let g = GIFT::Toy(String::from("Car"));

    if let GIFT::Toy(item) = g {
        println!("GOt {}", item);
    }
}
