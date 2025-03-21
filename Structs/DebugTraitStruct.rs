#[derive(Debug)]
struct Car {
    make: String,
    year: u32,
}

fn main() {
    let car = Car {
        make: String::from("Toyota"),
        year: 2020,
    };
    println!("{:?}", car);
}
