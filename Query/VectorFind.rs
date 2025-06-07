use std::num::ParseIntError;

fn main() {
    let Numbers  = vec![1, 2, 3, 4, 5];

    let sum = Numbers.iter().fold(0, |acc, &x| acc + x);
    println!("The sum of the numbers is: {}", sum);

    let found = Numbers.iter().find(|&&x| x == 3);
    println!("Found the number: {:?}", found);
}
