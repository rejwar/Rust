fn SumOfArray(arr: &[i32]) -> i32 {
    arr.iter().sum()
}

fn main() {
    let arr = [1, 2, 3, 4, 5];
    println!("Sum of array elements: {}", SumOfArray(&arr)); // Output: 15
}
