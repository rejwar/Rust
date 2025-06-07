fn main() {
    let vec = vec![1, 2, 3, 4, 5];
    let sum: i32 = vec.iter().sum();
    println!("The sum of the vector is: {}", sum);
    let average = sum as f64 / vec.len() as f64;
    println!("The average of the vector is: {}", average);  
}
