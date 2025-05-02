fn Double<T: std::ops::Add<Output = T> + Copy>(Num: T) -> T {
    Num + Num
}

fn main() {
    let Result = Double::<f64>(4.5);
    println!("Doubled Value: {}", Result);
}
