#[derive(Debug)]

struct Config {
    debug: bool,
}

impl Default for Config {
    fn default() -> Self {
        Config { debug: false }
    }
}