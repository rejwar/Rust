struct Config {
    debug: bool,
    max_connections: u32,
}

fn main() {
    let default_config = Config {
        debug: false,
        max_connections: 100,
    };

    let custom_config = Config {
        debug: true,
        ..default_config
    };

    println!("Debug: {}, Max Connections: {}", custom_config.debug, custom_config.max_connections);
}
