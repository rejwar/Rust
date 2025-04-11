// Main Function
fn main() {
    // Basic Floating Point Declaration
    let FloatValue: f32 = 3.14;
    let DoubleValue: f64 = 2.71828;

    // Scientific Notation
    let AvogadroNumber = 6.022e23; // f64 by default
    let ElectronMass = 9.109e-31;  // f64 by default

    // Arithmetic Operations
    let Sum = FloatValue + 1.0;
    let Product = DoubleValue * 2.0;
    let Division = 10.0 / 3.0;

    // Special Values
    let Infinity = f32::INFINITY;
    let NegativeInfinity = f64::NEG_INFINITY;
    let NaN = f32::NAN;

    // Type Conversion
    let IntToFloat = 5 as f32;
    let FloatToInt = 3.99 as i32; // Truncates to 3

    println!("Basic Values: {}, {}", FloatValue, DoubleValue);
    println!("Scientific: {}, {}", AvogadroNumber, ElectronMass);
    println!("Operations: {}, {}, {}", Sum, Product, Division);
    println!("Special: {}, {}, {}", Infinity, NegativeInfinity, NaN);
    println!("Conversions: {}, {}", IntToFloat, FloatToInt);

    // Floating Point Comparison (Caution with ==)
    if (1.0 + 2.0 - 3.0).abs() < f64::EPSILON {
        println!("Floating point comparison works with epsilon!");
    }
}
