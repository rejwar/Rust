fn main() {
    let numbers = vec![1, 2, 3];

    let mut iter = numbers.iter(); // Immutable iterator

    while let Some(n) = iter.next() {
        println!("Got: {}", n);
    }
}
