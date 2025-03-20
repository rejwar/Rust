const PI: f64 = 3.14159;

fn main() {
    let radius = 5.0;
    println!("Circumference: {}", calculate_circumference(radius));
}

fn calculate_circumference(radius: f64) -> f64 {
    2.0 * PI * radius
}
