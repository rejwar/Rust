mod alpha {
    pub fn signal() {
        println!("Signal From Alpha");
    }
    pub fn call_signal() {
        self::signal();
    }
}

fn main() {
    alpha::call_signal();
}
