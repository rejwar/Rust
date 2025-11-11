fn LargestValue<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut MaxValue = list[0];

    for &item in list {
        if item > MaxValue {
            MaxValue = item;
        }
    }

    MaxValue
}

fn main() {
    let numbers = vec![10, 20, 30, 40];
    println!("Largest = {}", LargestValue(&numbers));
}
