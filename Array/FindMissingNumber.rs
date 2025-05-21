fn FindMissingNumber(arr: &[i32], n: i32) -> i32 {
    let expected_sum = n * (n + 1) / 2;
    let actual_sum: i32 = arr.iter().sum();
    expected_sum - actual_sum
}

fn main() {
    let arr = [1, 2, 4, 5, 6];
    let n = 6;
    println!("Missing number: {}", FindMissingNumber(&arr, n)); // Output: 3
}
