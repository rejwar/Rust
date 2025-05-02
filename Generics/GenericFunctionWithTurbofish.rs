fn Square<T: std::ops::Mul<Output = T> + Copy>(Num: T) -> T {
    Num * Num
}

fn main() {
    let Result = Square::<i32>(5); // Using Turbofish
    println!("Squared Result: {}", Result);
}
