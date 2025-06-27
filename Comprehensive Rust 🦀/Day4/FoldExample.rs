// ❓ Question: How do you reduce a sequence into one value using .fold()?

fn main() {
    let nums = vec![1, 2, 3, 4];

    let sum = nums.iter().fold(0, |acc, x| acc + x);

    println!("Sum = {}", sum); // 10
}
