#[link(name = "m")]
extern "C" {
    fn sqrt(x: f64) -> f64;
}

fn main() {
    unsafe {
        println!("Square Root: {}", sqrt(16.0));
    }
}
