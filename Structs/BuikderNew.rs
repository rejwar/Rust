#[derive(Debug)]

struct Config {
    host: String,
    port: u16,
}

impl Config {
    fn new(host: impl Into <String> , port: u16  ) -> Self {
        Self { host: host.into(), port }
    }
}

fn main () {
    let cfg = Config::new("Local Host", 800);
    println!("{:?}", cfg);
}