#[derive(Default)]
struct Config {
    debug: bool,
    max_connections: u32,
}

fn main() {
    let config = Config::default();
    println!("Debug: {}, Max Connections: {}", config.debug, config.max_connections);
}
