fn main() {
    let numbers: Vec<i32> = vec![1, 2, 3, 5, 6, 7];
    let sum: i32 = numbers.iter().sum();

    println!("Sum :{}", sum);
}
