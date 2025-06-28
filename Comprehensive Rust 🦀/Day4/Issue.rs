fn main() {
    let numbers = vec![1,2,3,4,5,6];

    let doubled_numbers: Vec<i32> = numbers
    .into_iter().map(|n| n*2).collect();

    println!("{:?}",doubled_numbers);
}
