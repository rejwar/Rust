struct Config;

impl Config {
    fn max_connections() -> u32 {
        100
    }
}

fn main() {
    println!("Max connections: {}", Config::max_connections());
}
