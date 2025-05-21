fn FindMin(arr: &[i32]) -> i32 {
    *arr.iter().min().unwrap()
}

fn main() {
    let arr = [10, 20, 5, 30, 25];
    println!("Minimum element: {}", FindMin(&arr)); // Output: 5
}
