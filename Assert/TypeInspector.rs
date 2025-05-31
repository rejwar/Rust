fn main() {
    let x: i32 = 1_000_000; // Using underscores for readability
    let y: i32 = 2_000_000; // Using underscores for readability
    let z:f64 = 0.01;

    assert_eq!(type_of(&x), "_".to_string());
    println!("Suceess");

    fn type_of<T> (_: &T) -> String {
         format!("{}", std::any::type_name::<T>())
    }
}
