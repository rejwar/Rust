#[derive(Debug , Default)]

struct Settings {
    sound : bool,
    brightness: u8,
    username: String,
}

fn main() {
    let s = Settings {brightness: 8 , ..Default::default()};
    println!("{:?}", s);
}