mod outer {
    pub mod inner {
        pub fn hello() {
            println!("Hello from inner");
        }
        pub fn run() {
            self::hello();
        }
    }
}

fn main() {
    outer::inner::run();
}
