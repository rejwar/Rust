#[derive (Default)] 
struct Config {
    DebugMode: bool,
    TimeOut: u32 ,

}

fn main() {
    let DefaultConfig: Config = Config::default();
    println!("Debug Mode : {} , Time Out : {}" , DefaultConfig.DebugMode , DefaultConfig.TimeOut);
}
