#[derive(Debug)]

struct Kilometers(f64);

impl From<Meters> for Kilometers {
    fn from(value: Meters) -> Self {
        Kilometers(value.0 / 1000)
    }
}

#[derive(Debug)]

struct Meters(f64);

fn main() {
    let meters  = Meters(5000.0);
    let kilometers: Kilometers = meters.into();
    println!("Kilometers : {:?}", kilometers);
}
