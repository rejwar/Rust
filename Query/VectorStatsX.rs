fn main () {
    let numbers = vec![1, 2, 3, 4, 5];

    let sum = numbers.iter().fold(0, |acc, &x| acc + x);
    println!("The sum of the numbers is: {}", sum);
    let product = numbers.iter().fold(1, |acc, &x| acc * x);
    println!("The product of the numbers is: {}", product);
    let max = numbers.iter().fold(i32::MIN, |acc, &x| acc.max(x));
    println!("The maximum number is: {}", max);
    let min = numbers.iter().fold(i32::MAX, |acc, &x| acc.min(x));
    println!("The minimum number is: {}", min);
}
