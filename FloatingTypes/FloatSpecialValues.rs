fn main() {
    let inf = f64::INFINITY;
    let neg_inf = f64::NEG_INFINITY;
    let nan = f64::NAN;

    println!("Infinity: {}", inf);
    println!("Negative Infinity: {}", neg_inf);
    println!("NaN: {}", nan);

    println!("Is NaN? {}", nan.is_nan());
    println!("Is Finite? {}", 3.14_f64.is_finite());
}
