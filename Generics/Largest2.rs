fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    let NumberList = vec![34, 35, 36, 37, 38];
    let Result = largest(&NumberList);
    println!("The largest Number is {}", Result);

    let CharList = vec!['y', 'N', 'M', 'Z'];
    let Result = largest(&CharList);
    println!("The largest char is {}", Result);
}
