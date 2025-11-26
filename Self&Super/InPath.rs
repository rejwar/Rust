mod alpha {
    pub fn signal() {
        println!("Signal is from Alpha");
    }
    pub fn call_signal() {
        self::signal();
    }
}
fn main() {
    alpha::call_signal();
}
