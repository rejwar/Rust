fn main() {
    let nums = vec![1, 2, 3, 4, 5, 6];

    let evens: Vec<_> = nums
        .into_iter()
        .filter(|x| x % 2 == 0)
        .map(|x| x * 10)
        .collect();

    println!("{:?}", evens);
}
