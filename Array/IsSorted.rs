fn IsSorted(arr: &[i32]) -> bool {
    arr.windows(2).all(|w| w[0] <= w[1])
}

fn main() {
    let arr = [1, 2, 3, 4, 5];
    println!("Is sorted? {}", IsSorted(&arr)); // Output: true
}
