fn main() {
    let Numbers = vec![1, 2, 3, 4, 5];
    let Sum: i32 = Numbers.iter().fold(0, |acc, x| acc + x);

    println!("Sum: {}", Sum);
}
