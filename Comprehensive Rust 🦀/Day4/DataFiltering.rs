fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    /
    let evens: Vec<i32> = numbers.iter().filter(|x| x % 2 == 0).cloned().collect();

    println!("Evens: {:?}", evens); // [2, 4]
}
