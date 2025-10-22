fn main() {
    let square_sum: i32 = (1..=5).map(|x| x * x).sum();
    println!("Sum of Squares {}", square_sum);
}
