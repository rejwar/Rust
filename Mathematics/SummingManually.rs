fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6];
    let mut sum = 0;
    for n in numbers {
        sum += n;
    }

    println!("Sum is {}", sum);
}
