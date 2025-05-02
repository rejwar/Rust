fn main() {
    let Numbers: Vec<i32> = (1..=5).collect::<Vec<i32>>(); // Turbofish applied
    println!("Collected Numbers: {:?}", Numbers);
}
