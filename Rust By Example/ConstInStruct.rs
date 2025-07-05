struct Config;

impl Config {
    pub const Version: &'static str = "1.0.0";
}

fn main() {
    println!("Version {}", Config::Version);
}
