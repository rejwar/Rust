enum Light {
    Red,
    Yellow,
    Green,
}

fn main() {
    let current = Light::Red;

    match current {
        Light::Red => println!("STOP"),
        Light::Green => println!("Go"),
        Light::Yellow=> println!("REday"),

    }
}
