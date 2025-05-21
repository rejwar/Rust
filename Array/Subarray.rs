fn KadaneAlgorithm(arr: &[i32]) -> i32 {
    let mut max_sum = arr[0];
    let mut current_sum = arr[0];

    for &num in arr.iter().skip(1) {
        current_sum = current_sum.max(0) + num;
        max_sum = max_sum.max(current_sum);
    }

    max_sum
}

fn main() {
    let arr = [-2, 1, -3, 4, -1, 2, 1, -5, 4];
    println!("Maximum Subarray Sum: {}", KadaneAlgorithm(&arr)); // Output: 6
}
